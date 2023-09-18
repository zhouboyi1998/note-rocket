use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Error, Result};

// ----------------- TIMESTAMP -----------------

/**
 *开始时间戳 2023-09-15 00:00:00 (确定后不允许更改)
 */
const EPOCH: i64 = 1694707200;

/**
 * 容忍的最大时间回拨幅度 (单位: 毫秒)
 */
const MAX_TIME_BACK: i64 = 5;

// -------------------- BIT --------------------

/**
 * 序列号占用位数 (每毫秒可以生成 2^12 个ID)
 */
const SEQUENCE_BIT: i64 = 12;

/**
 * 工作机器ID占用位数 (每个数据中心可以部署 2^5 个工作机器)
 */
const WORKER_ID_BIT: i64 = 5;

/**
 * 数据中心ID占用位数 (一共 2^5 个数据中心)
 */
const DATACENTER_ID_BIT: i64 = 5;

// ------------- --- MASK / MAX ----------------

/**
 * 序列号掩码 (2^12 - 1)
 */
const SEQUENCE_MASK: i64 = (-1 ^ (-1 << SEQUENCE_BIT)) as i64;

/**
 * 工作机器ID掩码 (2^5 - 1)
 */
const WORKER_ID_MASK: i64 = (-1 ^ (-1 << WORKER_ID_BIT)) as i64;

/**
 * 数据中心ID掩码 (2^5 - 1)
 */
const DATACENTER_ID_MASK: i64 = (-1 ^ (-1 << DATACENTER_ID_BIT)) as i64;

// ------------------- OFFSET ------------------

/**
 * 工作机器ID向左偏移 12 位
 */
const WORKER_ID_SHIFT: i64 = SEQUENCE_BIT;

/**
 * 数据中心ID向左偏移 17 位
 */
const DATACENTER_ID_SHIFT: i64 = SEQUENCE_BIT + WORKER_ID_BIT;

/**
 * 时间戳向左偏移 22 位
 */
const TIMESTAMP_SHIFT: i64 = SEQUENCE_BIT + WORKER_ID_BIT + DATACENTER_ID_BIT;

// ------------------- FIELD -------------------

pub struct Snowflake {
    /**
     * 序列号
     */
    sequence: i64,

    /**
     * 工作机器ID
     */
    worker_id: i64,

    /**
     * 数据中心ID
     */
    datacenter_id: i64,

    /**
     * 最后一次生成分布式ID的时间戳
     */
    last_timestamp: i64,
}

impl Snowflake {
    /**
     * 新建 Snowflake 对象
     */
    pub fn new(worker_id: i64, datacenter_id: i64) -> Result<Snowflake> {
        // 工作机器ID超出限制
        if worker_id > WORKER_ID_MASK {
            return Err(Error::msg(format!("worker_id: {} must be less than {} !", worker_id, WORKER_ID_MASK)));
        }
        // 数据中心ID超出限制
        if datacenter_id > DATACENTER_ID_MASK {
            return Err(Error::msg(format!("datacenterId: {} must be less than {} !", datacenter_id, DATACENTER_ID_MASK)));
        }

        Ok(Snowflake { worker_id, datacenter_id, sequence: 0, last_timestamp: 0 })
    }

    /**
     * 获取下一个分布式ID
     */
    pub fn next_id(&mut self) -> Result<i64> {
        // 获取当前时间戳
        let mut timestamp = Self::time_gen()?;

        // 如果当前时间戳小于最后一次生成分布式ID的时间戳, 说明发生了时间回拨
        if timestamp < self.last_timestamp {
            // 获取时间回拨幅度
            let offset = self.last_timestamp - timestamp;
            // 容忍的最大时间回拨幅度
            if offset <= MAX_TIME_BACK {
                // 等待双倍时间回拨幅度的时间, 再开始生成分布式ID
                thread::sleep(Duration::new((offset << 1) as u64, 0));
                timestamp = Self::time_gen()?;
                // 再次发生时钟回拨
                if timestamp < self.last_timestamp {
                    return Err(Error::msg(format!("Thead wait {} milliseconds. Clock moved backwards again. Refusing to generate id for {} milliseconds.", MAX_TIME_BACK, self.last_timestamp - timestamp)));
                }
            } else {
                return Err(Error::msg(format!("Clock moved backwards. Refusing to generate id for {} milliseconds.", self.last_timestamp - timestamp)));
            }
        }

        // 如果当前时间戳等于最后一次生成分布式ID的时间戳 (同一毫秒内), 序列号递增, 生成新的分布式ID
        if timestamp == self.last_timestamp {
            // 序列号递增
            self.sequence = (self.sequence + 1) & SEQUENCE_MASK;
            // 达到毫秒内序列号的最大值, 阻塞线程直到下一毫秒
            if self.sequence == 0 {
                // 接收下一毫秒的时间戳
                timestamp = Self::til_next_millis(self.last_timestamp)?;
            }
        } else {
            // 如果前时间戳等于最后一次生成分布式ID的时间戳 (时间已经走到下一毫秒), 重置序列号
            self.sequence = 0;
        }

        // 更新最后一次生成分布式ID的时间戳
        self.last_timestamp = timestamp;

        // 通过移位 + 或运算, 将 4 个部分拼接成 64bit 的雪花分布式ID
        Ok(((timestamp - EPOCH) << TIMESTAMP_SHIFT)
            | (self.datacenter_id << DATACENTER_ID_SHIFT)
            | (self.worker_id << WORKER_ID_SHIFT)
            | self.sequence)
    }

    /**
     * 阻塞线程直到下一毫秒
     */
    fn til_next_millis(last_timestamp: i64) -> Result<i64> {
        let mut timestamp = Self::time_gen()?;
        // 当时间走到下一毫秒, 返回下一毫秒的时间戳
        while timestamp <= last_timestamp {
            timestamp = Self::time_gen()?;
        }
        Ok(timestamp)
    }

    /**
     * 获取当前时间戳
     */
    fn time_gen() -> Result<i64> {
        // 获取系统时间
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(s) => { Ok(s.as_millis() as i64) }
            Err(_) => { Err(Error::msg("Get system time error!")) }
        }
    }
}
