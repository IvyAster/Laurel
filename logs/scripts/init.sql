CREATE
DATABASE laurel_logs
    WITH
    OWNER = admin
    ENCODING = 'UTF8'
    LC_COLLATE = 'en_US.UTF-8'
    LC_CTYPE = 'en_US.UTF-8'
    TEMPLATE = template0;

CREATE TABLE login_log(
                          id BIGSERIAL NOT NULL PRIMARY KEY,
                          ticket_id VARCHAR(40) NOT NULL,
                          account VARCHAR(40) NOT NULL,
                          login_type    VARCHAR(20) NOT NULL,
                          login_state VARCHAR(20) NOT NULL,
                          login_result VARCHAR(200) DEFAULT NULL,
                          ip VARCHAR(64) DEFAULT NULL,
                          location VARCHAR(128) DEFAULT NULL,
                          browser VARCHAR(128) DEFAULT NULL,
                          os VARCHAR(128) DEFAULT NULL,
                          device VARCHAR(128) DEFAULT NULL,
                          cts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                          login_cts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                          CONSTRAINT uniq_ti UNIQUE (ticket_id)
);

CREATE INDEX login_log_idx_account ON login_log (account);
CREATE INDEX login_log_idx_login_status ON login_log (login_state);
CREATE INDEX login_log_idx_ip ON login_log (ip);