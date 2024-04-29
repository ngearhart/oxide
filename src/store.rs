
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::serialization::NULL;

lazy_static! {
    /// Global data storage for key/value pairs. Thread safe.
    static ref HASHMAP: Mutex<HashMap<String, String>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };

    /// Global configuration storage. Thread safe.
    static ref CONFIG: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        // Defaults
        populate_config_defaults(&mut m);
        Mutex::new(m)
    };
}

pub fn global_store_set(key: String, value: String) {
    let mut hashmap = HASHMAP.lock().unwrap();
    hashmap.insert(key, value);
}

pub fn global_store_get(key: String) -> String {
    let hashmap = HASHMAP.lock().unwrap();
    hashmap.get(&key).unwrap_or(&NULL.to_owned()).to_string()
}

// Not implemented.
// pub fn global_config_set(key: String, value: String) {
//     let mut hashmap = CONFIG.lock().unwrap();
//     hashmap.insert(key, value);
// }

pub fn global_config_get(key: String) -> String {
    let hashmap = CONFIG.lock().unwrap();
    hashmap.get(&key).unwrap_or(&NULL.to_owned()).to_string()
}

pub fn global_config_get_keys(match_text: String) -> Vec<String> {
    let hashmap = CONFIG.lock().unwrap();
    if match_text.contains("*") {
        let regex_to_match = Regex::new(&match_text).unwrap();
        return hashmap.keys().filter(|key| regex_to_match.is_match(key)).map(|s| s.to_string()).collect();
    }
    hashmap.keys().find(|key| key.to_string() == match_text).into_iter().map(|x| x.to_string()).collect::<Vec<String>>()
}

/// Insert a bunch of config defaults based on with what a vanilla Redis server would respond.
/// I know this is ugly. Sorry.
fn populate_config_defaults(m: &mut HashMap<String, String>) {
    m.insert("replica-read-only".to_owned(), "yes".to_owned());
    m.insert("stream-node-max-bytes".to_owned(), "4096".to_owned());
    m.insert("auto-aof-rewrite-percentage".to_owned(), "100".to_owned());
    m.insert("bind-source-addr".to_owned(), "".to_owned());
    m.insert("tls-client-key-file".to_owned(), "".to_owned());
    m.insert("notify-keyspace-events".to_owned(), "".to_owned());
    m.insert("set-max-intset-entries".to_owned(), "512".to_owned());
    m.insert("slowlog-log-slower-than".to_owned(), "10000".to_owned());
    m.insert("cluster-slave-no-failover".to_owned(), "no".to_owned());
    m.insert("syslog-facility".to_owned(), "local0".to_owned());
    m.insert("tls-cert-file".to_owned(), "".to_owned());
    m.insert("masterauth".to_owned(), "".to_owned());
    m.insert("repl-diskless-load".to_owned(), "disabled".to_owned());
    m.insert("loglevel".to_owned(), "notice".to_owned());
    m.insert("repl-ping-replica-period".to_owned(), "10".to_owned());
    m.insert("tls-ca-cert-dir".to_owned(), "".to_owned());
    m.insert("cluster-migration-barrier".to_owned(), "1".to_owned());
    m.insert("proc-title-template".to_owned(), "{title} {listen-addr} {server-mode}".to_owned());
    m.insert("cluster-config-file".to_owned(), "nodes.conf".to_owned());
    m.insert("tls-ciphers".to_owned(), "".to_owned());
    m.insert("replica-announce-ip".to_owned(), "".to_owned());
    m.insert("rdbcompression".to_owned(), "yes".to_owned());
    m.insert("cluster-announce-tls-port".to_owned(), "0".to_owned());
    m.insert("tls-dh-params-file".to_owned(), "".to_owned());
    m.insert("maxclients".to_owned(), "10000".to_owned());
    m.insert("aof-load-truncated".to_owned(), "yes".to_owned());
    m.insert("hll-sparse-max-bytes".to_owned(), "3000".to_owned());
    m.insert("cluster-link-sendbuf-limit".to_owned(), "0".to_owned());
    m.insert("maxmemory".to_owned(), "0".to_owned());
    m.insert("bind".to_owned(), "* -::*".to_owned());
    m.insert("aof-timestamp-enabled".to_owned(), "no".to_owned());
    m.insert("hash-max-ziplist-entries".to_owned(), "512".to_owned());
    m.insert("supervised".to_owned(), "no".to_owned());
    m.insert("active-defrag-cycle-max".to_owned(), "25".to_owned());
    m.insert("tls-client-key-file-pass".to_owned(), "".to_owned());
    m.insert("masteruser".to_owned(), "".to_owned());
    m.insert("zset-max-ziplist-value".to_owned(), "64".to_owned());
    m.insert("list-max-ziplist-size".to_owned(), "-2".to_owned());
    m.insert("repl-ping-slave-period".to_owned(), "10".to_owned());
    m.insert("min-slaves-max-lag".to_owned(), "10".to_owned());
    m.insert("oom-score-adj-values".to_owned(), "0 200 800".to_owned());
    m.insert("tls-session-caching".to_owned(), "yes".to_owned());
    m.insert("lfu-decay-time".to_owned(), "1".to_owned());
    m.insert("timeout".to_owned(), "0".to_owned());
    m.insert("min-replicas-to-write".to_owned(), "0".to_owned());
    m.insert("maxmemory-clients".to_owned(), "0".to_owned());
    m.insert("cluster-announce-human-nodename".to_owned(), "".to_owned());
    m.insert("cluster-preferred-endpoint-type".to_owned(), "ip".to_owned());
    m.insert("active-defrag-ignore-bytes".to_owned(), "104857600".to_owned());
    m.insert("pidfile".to_owned(), "".to_owned());
    m.insert("port".to_owned(), "6379".to_owned());
    m.insert("slave-read-only".to_owned(), "yes".to_owned());
    m.insert("maxmemory-policy".to_owned(), "noeviction".to_owned());
    m.insert("tls-port".to_owned(), "0".to_owned());
    m.insert("set-max-listpack-value".to_owned(), "64".to_owned());
    m.insert("propagation-error-behavior".to_owned(), "ignore".to_owned());
    m.insert("client-query-buffer-limit".to_owned(), "1073741824".to_owned());
    m.insert("active-defrag-max-scan-fields".to_owned(), "1000".to_owned());
    m.insert("tls-protocols".to_owned(), "".to_owned());
    m.insert("oom-score-adj".to_owned(), "no".to_owned());
    m.insert("proto-max-bulk-len".to_owned(), "536870912".to_owned());
    m.insert("aof-use-rdb-preamble".to_owned(), "yes".to_owned());
    m.insert("aof_rewrite_cpulist".to_owned(), "".to_owned());
    m.insert("save".to_owned(), "3600 1 300 100 60 10000".to_owned());
    m.insert("list-compress-depth".to_owned(), "0".to_owned());
    m.insert("databases".to_owned(), "16".to_owned());
    m.insert("cluster-node-timeout".to_owned(), "15000".to_owned());
    m.insert("busy-reply-threshold".to_owned(), "5000".to_owned());
    m.insert("maxmemory-eviction-tenacity".to_owned(), "10".to_owned());
    m.insert("rdbchecksum".to_owned(), "yes".to_owned());
    m.insert("cluster-port".to_owned(), "0".to_owned());
    m.insert("repl-disable-tcp-nodelay".to_owned(), "no".to_owned());
    m.insert("cluster-replica-no-failover".to_owned(), "no".to_owned());
    m.insert("ignore-warnings".to_owned(), "".to_owned());
    m.insert("daemonize".to_owned(), "no".to_owned());
    m.insert("appenddirname".to_owned(), "appendonlydir".to_owned());
    m.insert("activerehashing".to_owned(), "yes".to_owned());
    m.insert("lfu-log-factor".to_owned(), "10".to_owned());
    m.insert("list-max-listpack-size".to_owned(), "-2".to_owned());
    m.insert("cluster-slave-validity-factor".to_owned(), "10".to_owned());
    m.insert("io-threads-do-reads".to_owned(), "no".to_owned());
    m.insert("tls-key-file-pass".to_owned(), "".to_owned());
    m.insert("auto-aof-rewrite-min-size".to_owned(), "67108864".to_owned());
    m.insert("dynamic-hz".to_owned(), "yes".to_owned());
    m.insert("set-proc-title".to_owned(), "yes".to_owned());
    m.insert("unixsocketperm".to_owned(), "0".to_owned());
    m.insert("dbfilename".to_owned(), "dump.rdb".to_owned());
    m.insert("cluster-replica-validity-factor".to_owned(), "10".to_owned());
    m.insert("cluster-allow-reads-when-down".to_owned(), "no".to_owned());
    m.insert("active-expire-effort".to_owned(), "1".to_owned());
    m.insert("cluster-require-full-coverage".to_owned(), "yes".to_owned());
    m.insert("latency-monitor-threshold".to_owned(), "0".to_owned());
    m.insert("tls-auth-clients".to_owned(), "yes".to_owned());
    m.insert("tls-client-cert-file".to_owned(), "".to_owned());
    m.insert("replica-lazy-flush".to_owned(), "no".to_owned());
    m.insert("replica-priority".to_owned(), "100".to_owned());
    m.insert("slave-announce-ip".to_owned(), "".to_owned());
    m.insert("tls-replication".to_owned(), "no".to_owned());
    m.insert("cluster-allow-replica-migration".to_owned(), "yes".to_owned());
    m.insert("enable-debug-command".to_owned(), "no".to_owned());
    m.insert("tls-key-file".to_owned(), "".to_owned());
    m.insert("latency-tracking".to_owned(), "yes".to_owned());
    m.insert("slave-ignore-maxmemory".to_owned(), "yes".to_owned());
    m.insert("hash-max-listpack-value".to_owned(), "64".to_owned());
    m.insert("rdb-save-incremental-fsync".to_owned(), "yes".to_owned());
    m.insert("always-show-logo".to_owned(), "no".to_owned());
    m.insert("bio_cpulist".to_owned(), "".to_owned());
    m.insert("server_cpulist".to_owned(), "".to_owned());
    m.insert("tcp-backlog".to_owned(), "511".to_owned());
    m.insert("rdb-del-sync-files".to_owned(), "no".to_owned());
    m.insert("lazyfree-lazy-expire".to_owned(), "no".to_owned());
    m.insert("dir".to_owned(), "/data".to_owned());
    m.insert("io-threads".to_owned(), "1".to_owned());
    m.insert("active-defrag-threshold-lower".to_owned(), "10".to_owned());
    m.insert("cluster-allow-pubsubshard-when-down".to_owned(), "yes".to_owned());
    m.insert("logfile".to_owned(), "".to_owned());
    m.insert("enable-protected-configs".to_owned(), "no".to_owned());
    m.insert("zset-max-listpack-value".to_owned(), "64".to_owned());
    m.insert("requirepass".to_owned(), "".to_owned());
    m.insert("hz".to_owned(), "10".to_owned());
    m.insert("appendfilename".to_owned(), "appendonly.aof".to_owned());
    m.insert("tls-ciphersuites".to_owned(), "".to_owned());
    m.insert("aof-rewrite-incremental-fsync".to_owned(), "yes".to_owned());
    m.insert("lazyfree-lazy-server-del".to_owned(), "no".to_owned());
    m.insert("cluster-announce-bus-port".to_owned(), "0".to_owned());
    m.insert("active-defrag-threshold-upper".to_owned(), "100".to_owned());
    m.insert("acl-pubsub-default".to_owned(), "resetchannels".to_owned());
    m.insert("lazyfree-lazy-user-del".to_owned(), "no".to_owned());
    m.insert("shutdown-timeout".to_owned(), "10".to_owned());
    m.insert("unixsocket".to_owned(), "".to_owned());
    m.insert("cluster-announce-ip".to_owned(), "".to_owned());
    m.insert("slave-announce-port".to_owned(), "0".to_owned());
    m.insert("cluster-enabled".to_owned(), "no".to_owned());
    m.insert("tls-cluster".to_owned(), "no".to_owned());
    m.insert("cluster-announce-hostname".to_owned(), "".to_owned());
    m.insert("hash-max-ziplist-value".to_owned(), "64".to_owned());
    m.insert("repl-backlog-size".to_owned(), "1048576".to_owned());
    m.insert("protected-mode".to_owned(), "no".to_owned());
    m.insert("activedefrag".to_owned(), "no".to_owned());
    m.insert("slave-lazy-flush".to_owned(), "no".to_owned());
    m.insert("latency-tracking-info-percentiles".to_owned(), "50 99 99.9".to_owned());
    m.insert("sanitize-dump-payload".to_owned(), "no".to_owned());
    m.insert("maxmemory-samples".to_owned(), "5".to_owned());
    m.insert("socket-mark-id".to_owned(), "0".to_owned());
    m.insert("crash-memcheck-enabled".to_owned(), "yes".to_owned());
    m.insert("replica-announce-port".to_owned(), "0".to_owned());
    m.insert("appendonly".to_owned(), "no".to_owned());
    m.insert("active-defrag-cycle-min".to_owned(), "1".to_owned());
    m.insert("slave-priority".to_owned(), "100".to_owned());
    m.insert("bgsave_cpulist".to_owned(), "".to_owned());
    m.insert("replica-announced".to_owned(), "yes".to_owned());
    m.insert("aclfile".to_owned(), "".to_owned());
    m.insert("lazyfree-lazy-user-flush".to_owned(), "no".to_owned());
    m.insert("crash-log-enabled".to_owned(), "yes".to_owned());
    m.insert("min-replicas-max-lag".to_owned(), "10".to_owned());
    m.insert("slowlog-max-len".to_owned(), "128".to_owned());
    m.insert("shutdown-on-sigterm".to_owned(), "default".to_owned());
    m.insert("repl-diskless-sync-delay".to_owned(), "5".to_owned());
    m.insert("tcp-keepalive".to_owned(), "300".to_owned());
    m.insert("repl-timeout".to_owned(), "60".to_owned());
    m.insert("tls-session-cache-size".to_owned(), "20480".to_owned());
    m.insert("replica-ignore-disk-write-errors".to_owned(), "no".to_owned());
    m.insert("tls-prefer-server-ciphers".to_owned(), "no".to_owned());
    m.insert("zset-max-listpack-entries".to_owned(), "128".to_owned());
    m.insert("tracking-table-max-keys".to_owned(), "1000000".to_owned());
    m.insert("replica-serve-stale-data".to_owned(), "yes".to_owned());
    m.insert("tls-ca-cert-file".to_owned(), "".to_owned());
    m.insert("locale-collate".to_owned(), "".to_owned());
    m.insert("client-output-buffer-limit".to_owned(), "normal 0 0 0 slave 268435456 67108864 60 pubsub 33554432 8388608 60".to_owned());
    m.insert("jemalloc-bg-thread".to_owned(), "yes".to_owned());
    m.insert("set-max-listpack-entries".to_owned(), "128".to_owned());
    m.insert("replicaof".to_owned(), "".to_owned());
    m.insert("enable-module-command".to_owned(), "no".to_owned());
    m.insert("lua-time-limit".to_owned(), "5000".to_owned());
    m.insert("syslog-enabled".to_owned(), "no".to_owned());
    m.insert("replica-ignore-maxmemory".to_owned(), "yes".to_owned());
    m.insert("tls-session-cache-timeout".to_owned(), "300".to_owned());
    m.insert("lazyfree-lazy-eviction".to_owned(), "no".to_owned());
    m.insert("repl-diskless-sync".to_owned(), "yes".to_owned());
    m.insert("shutdown-on-sigint".to_owned(), "default".to_owned());
    m.insert("disable-thp".to_owned(), "yes".to_owned());
    m.insert("appendfsync".to_owned(), "everysec".to_owned());
    m.insert("cluster-announce-port".to_owned(), "0".to_owned());
    m.insert("no-appendfsync-on-rewrite".to_owned(), "no".to_owned());
    m.insert("zset-max-ziplist-entries".to_owned(), "128".to_owned());
    m.insert("acllog-max-len".to_owned(), "128".to_owned());
    m.insert("hash-max-listpack-entries".to_owned(), "512".to_owned());
    m.insert("syslog-ident".to_owned(), "redis".to_owned());
    m.insert("repl-diskless-sync-max-replicas".to_owned(), "0".to_owned());
    m.insert("stream-node-max-entries".to_owned(), "100".to_owned());
    m.insert("repl-backlog-ttl".to_owned(), "3600".to_owned());
    m.insert("stop-writes-on-bgsave-error".to_owned(), "yes".to_owned());
    m.insert("slaveof".to_owned(), "".to_owned());
    m.insert("slave-serve-stale-data".to_owned(), "yes".to_owned());
    m.insert("min-slaves-to-write".to_owned(), "0".to_owned());
}