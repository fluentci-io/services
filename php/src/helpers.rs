use std::vec;

use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_flox() -> Result<(), Error> {
    let os = dag().get_os()?;
    if os == "macos" {
        dag()
        .pipeline("setup-flox")?
        .with_exec(vec![r#"type brew > /dev/null 2> /dev/null || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#])?
        .with_exec(vec!["type flox > /dev/null 2> /dev/null || brew install flox"])?
        .stdout()?;
    }
    Ok(())
}

pub fn setup() -> Result<String, Error> {
    setup_flox()?;
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/log"])?
        .with_exec(vec![
            "grep -q log .fluentci/.gitignore || echo 'log/' >> .fluentci/.gitignore",
        ])?
        .stdout()?;
    let pwd = dag().get_env("PWD")?;
    let phprc = dag().get_env("PHPRC")?;
    let phpfpm_port = dag().get_env("PHPFPM_PORT")?;
    let phpfpm_pid_file = dag().get_env("PHPFPM_PID_FILE")?;
    let phpfpm_error_log_file = dag().get_env("PHPFPM_ERROR_LOG_FILE")?;

    if phprc.is_empty() {
        dag().set_envs(vec![("PHPRC".into(), pwd.clone())])?;
    }
    if phpfpm_port.is_empty() {
        dag().set_envs(vec![("PHPFPM_PORT".into(), "9000".into())])?;
    }

    if phpfpm_pid_file.is_empty() {
        dag().set_envs(vec![(
            "PHPFPM_PID_FILE".into(),
            format!("{}/.fluentci/php-fpm.pid", pwd),
        )])?;
    }

    if phpfpm_error_log_file.is_empty() {
        dag().set_envs(vec![(
            "PHPFPM_ERROR_LOG_FILE".into(),
            format!("{}/.fluentci/log/php-fpm.error.log", pwd),
        )])?;
    }

    let php_extensions = dag().get_env("PHP_EXTENSIONS")?;
    if php_extensions.is_empty() {
        dag().set_envs(vec![(
            "PHP_EXTENSIONS".into(),
            "imagick ds pgsql xdebug".into(),
        )])?;
    }

    let php_extensions = dag()
        .get_env("PHP_EXTENSIONS")?
        .split_whitespace()
        .map(|ext| format!("php83Extensions.{}", ext))
        .collect::<Vec<String>>()
        .join(" ");

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec![
            "flox",
            "install",
            &php_extensions,
            /*
            "php83Extensions.phalcon",
            "php83Extensions.imagick",
            "php83Extensions.snuffleupagus",
            "php83Extensions.openswoole",
            "php83Extensions.spx",
            "php83Extensions.php-spx",
            "php83Extensions.memcached",
            "php83Extensions.memprof",
            "php83Extensions.pcov",
            "php83Extensions.gnupg",
            "php83Extensions.amqp",
            "php83Extensions.ds",
            "php83Extensions.ast",
            "php83Extensions.opentelemetry",
            "php83Extensions.meminfo",
            "php83Extensions.msgpack",
            "php83Extensions.memcache",
            "php83Extensions.swoole",
            "php83Extensions.pdo_sqlsrv",
            "php83Extensions.sqlsrv",
            "php83Extensions.redis",
            "php83Extensions.relay",
            "php83Extensions.openssl-legacy",
            "php83Extensions.ssh2",
            "php83Extensions.pdo_sqlite",
            "php83Extensions.pdo_mysql",
            "php83Extensions.pdo_pgsql",
            "php83Extensions.simplexml",
            "php83Extensions.tokenizer",
            "php83Extensions.xmlreader",
            "php83Extensions.xmlwriter",
            "php83Extensions.zend_test",
            "php83Extensions.calendar",
            "php83Extensions.fileinfo",
            "php83Extensions.mbstring",
            "php83Extensions.pdo_odbc",
            "php83Extensions.readline",
            "php83Extensions.enchant",
            "php83Extensions.gettext",
            "php83Extensions.mongodb",
            "php83Extensions.mysqlnd",
            "php83Extensions.opcache",
            "php83Extensions.openssl",
            "php83Extensions.session",
            "php83Extensions.sockets",
            "php83Extensions.sqlite3",
            "php83Extensions.sysvsem",
            "php83Extensions.sysvshm",
            "php83Extensions.rrd",
            "php83Extensions.couchbase",
            "php83Extensions.bcmath",
            "php83Extensions.filter",
            "php83Extensions.mysqli",
            "php83Extensions.pspell",
            "php83Extensions.sodium",
            "php83Extensions.blackfire",
            "php83Extensions.ctype",
            "php83Extensions.iconv",
            "php83Extensions.pcntl",
            "php83Extensions.pgsql",
            "php83Extensions.posix",
            "php83Extensions.shmop",
            "php83Extensions.smbclient",
            "php83Extensions.igbinary",
            "php83Extensions.curl",
            "php83Extensions.exif",
            "php83Extensions.imap",
            "php83Extensions.intl",
            "php83Extensions.ldap",
            "php83Extensions.soap",
            "php83Extensions.tidy",
            "php83Extensions.zlib",
            "php83Extensions.bz2",
            "php83Extensions.dba",
            "php83Extensions.dom",
            "php83Extensions.ffi",
            "php83Extensions.ftp",
            "php83Extensions.gmp",
            "php83Extensions.pdo",
            "php83Extensions.xml",
            "php83Extensions.xsl",
            "php83Extensions.zip",
            "php83Extensions.datadog_trace",
            "php83Extensions.gd",
            "php83Extensions.pinba",
            "php83Extensions.pdlib",
            "php83Extensions.apcu",
            "php83Extensions.zstd",*/
            "php83Packages.composer",
            "php83",
            "overmind",
            "tmux",
            "wget",
            "curl",
        ])?
        .with_exec(vec!["cp ../composer.json ."])?
        .with_exec(vec!["cp ../composer.lock ."])?
        .with_exec(vec!["[ -f ../php.ini ] || wget https://raw.githubusercontent.com/fluentci-io/services/main/php/php.ini -O ../php.ini"])?
        .with_exec(vec![r#"grep -q extension_dir ../php.ini || echo -e "\nextension_dir = \"$(ls -d .flox/run/*/lib/php/extensions)\"" >> ../php.ini"#])?
        .with_exec(vec!["composer", "install"])?
        .with_exec(vec!["rm -rf ../vendor && mv vendor .."])?
        .with_exec(vec!["[ -f ../php-fpm.conf ] || wget https://raw.githubusercontent.com/fluentci-io/services/main/php/php-fpm.conf -O ../php-fpm.conf"])?
        .with_exec(vec![
            "grep -q php-fpm Procfile || echo -e 'php-fpm: php-fpm -y ../php-fpm.conf --nodaemonize\\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
