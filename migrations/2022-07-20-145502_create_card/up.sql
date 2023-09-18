CREATE TABLE note_card
(
    id          BIGINT PRIMARY KEY NOT NULL,
    title       VARCHAR            NOT NULL,
    content     VARCHAR            NOT NULL,
    tip         VARCHAR            NOT NULL,
    extra       VARCHAR            NOT NULL,
    create_time TIMESTAMP          NOT NULL
)
