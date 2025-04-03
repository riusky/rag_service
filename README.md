## 启动数据库

```sh
# 启动PostgreSQL服务器Docker镜像:
docker run --rm --name pg -p 5433:5432 -e POSTGRES_PASSWORD=welcome postgres:17

# (可选)在pg容器中启动psql终端
# 在另一个终端(标签页)中运行psql:
docker exec -it -u postgres pg psql

# (可选)让pg打印所有SQL语句
# 在上方启动的psql命令行中执行:
ALTER DATABASE postgres SET log_statement = 'all';
```

## 开发模式(监控变化)

> 注意: 使用`cargo install cargo-watch`安装cargo watch工具

```sh
# 终端1 - 运行服务器
cargo watch -q -c -w crates/services/web-server/src/ -w crates/libs/ -w .cargo/ -x "run -p web-server"

# 终端2 - 运行quick_dev示例
cargo watch -q -c -w crates/services/web-server/examples/ -x "run -p web-server --example quick_dev"
```

## 开发

```sh
# 终端1 - 运行服务器
cargo run -p web-server

# 终端2 - 运行测试
cargo run -p web-server --example quick_dev
```

## 单元测试(监控变化)

```sh
cargo watch -q -c -x "test -- --nocapture"

# 使用过滤器运行特定测试
cargo watch -q -c -x "test -p lib-core test_create -- --nocapture"
```

## 单元测试

```sh
cargo test -- --nocapture

cargo watch -q -c -x "test -p lib-core model::task::tests::test_create -- --nocapture"
```

## 工具

```sh
cargo run -p gen-key
```

<br />

---