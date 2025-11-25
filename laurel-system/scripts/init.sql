CREATE
    DATABASE laurel_system
    WITH
    OWNER = admin
    ENCODING = 'UTF8'
    LC_COLLATE = 'en_US.UTF-8'
    LC_CTYPE = 'en_US.UTF-8'
    TEMPLATE = template0;


CREATE TABLE fe_micro_service
(
    id             BIGSERIAL    NOT NULL PRIMARY KEY,
    app_id         VARCHAR(40)  NOT NULL,
    service_id     VARCHAR(40)  NOT NULL,
    service_name   VARCHAR(64)  NOT NULL,
    service_entry  VARCHAR(64)  NOT NULL,
    mount_point    VARCHAR(64)  NOT NULL,
    route_pattern  VARCHAR(100) NOT NULL,
    service_status VARCHAR(20)  NOT NULL,
    cts            TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    uts            TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uniq_ai_si UNIQUE (app_id, service_id),
    CONSTRAINT uniq_si UNIQUE (service_id)
);

COMMENT
    ON TABLE fe_micro_service IS '前端微服务表';

INSERT INTO fe_micro_service (app_id, service_id, service_name, service_entry, mount_point, route_pattern,
                              service_status)
VALUES ('data-map', '1987051901912485888', 'hzl-system', '//127.0.0.1:80', '#sub-app-view', '/hzl-system', 'open');

CREATE TABLE menu
(
    id               BIGSERIAL   NOT NULL PRIMARY KEY,
    app_id           VARCHAR(40) NOT NULL,
    menu_id          VARCHAR(40) NOT NULL,
    menu_name        VARCHAR(64) NOT NULL,
    menu_type        VARCHAR(40) NOT NULL,
    menu_action_type VARCHAR(40) NOT NULL,
    menu_icon        VARCHAR(60)          DEFAULT NULL,
    menu_route       VARCHAR(400)         DEFAULT NULL,
    route_param      TEXT                 DEFAULT NULL,
    weight           INTEGER     NOT NULL,
    parent_id        VARCHAR(40) NOT NULL,
    authority        VARCHAR(100)         DEFAULT NULL,
    menu_status      VARCHAR(20) NOT NULL,
    cts              TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    uts              TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uniq_ai_mi UNIQUE (app_id, menu_id),
    CONSTRAINT uniq_mi UNIQUE (menu_id)
);
COMMENT
    ON TABLE menu IS '应用菜单表';
CREATE INDEX idx_menu_pi ON menu (parent_id);


CREATE TABLE role
(
    id          BIGSERIAL   NOT NULL PRIMARY KEY,
    role_id     VARCHAR(40) NOT NULL,
    role_name   VARCHAR(64) NOT NULL,
    role_type   VARCHAR(40) NOT NULL,
    weight      INTEGER     NOT NULL,
    role_status VARCHAR(20) NOT NULL,
    cts         TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    uts         TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uniq_ri UNIQUE (role_id)
);
COMMENT
    ON TABLE role IS '角色表';

CREATE TABLE role_account
(
    id         BIGSERIAL   NOT NULL PRIMARY KEY,
    role_id    VARCHAR(40) NOT NULL,
    account_id VARCHAR(40) NOT NULL,
    cts        TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    uts        TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uniq_ri_ai UNIQUE (role_id, account_id),
    CONSTRAINT uniq_ai_ri UNIQUE (account_id, role_id)
);
COMMENT
    ON TABLE role_account IS '角色-账户表';


CREATE TABLE permission
(
    id      BIGSERIAL   NOT NULL PRIMARY KEY,
    role_id VARCHAR(40) NOT NULL,
    menu_id VARCHAR(40) NOT NULL,
    cts     TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    uts     TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uniq_ri UNIQUE (role_id)
);
COMMENT
    ON TABLE permission IS '权限表';


CREATE TABLE dict
(
    id        BIGSERIAL    NOT NULL PRIMARY KEY,
    dict_id   VARCHAR(64)  NOT NULL,
    dict_name VARCHAR(200) NOT NULL,
    dict_mark TEXT                  DEFAULT NULL,
    weight    INTEGER      NOT NULL,
    dict_type VARCHAR(40)  NOT NULL,
    cts       TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    uts       TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uniq_di UNIQUE (dict_id)
);
COMMENT
    ON TABLE dict IS '字典表';


CREATE TABLE dict_value
(
    id         BIGSERIAL    NOT NULL PRIMARY KEY,
    dict_id    VARCHAR(64)  NOT NULL,
    value_id   VARCHAR(64)  NOT NULL,
    value_name VARCHAR(200) NOT NULL,
    value_mark  TEXT                  DEFAULT NULL,
    weight     INTEGER      NOT NULL,
    dict_type  VARCHAR(40)  NOT NULL,
    cts        TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    uts        TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uniq_di_vi UNIQUE (dict_id, value_id)
);
COMMENT
    ON TABLE dict_value IS '字典值表';