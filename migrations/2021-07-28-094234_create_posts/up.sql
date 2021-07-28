CREATE TABLE posts(
       id              INTEGER   NOT NULL PRIMARY KEY      AUTOINCREMENT ,
       title           TEXT    NOT NULL,
       path            TEXT     NOT NULL,
       intro        CHAR(50)    ,
       time         TIMESTAMP  DEFAULT (datetime('now', 'localtime')) NOT NULL
);