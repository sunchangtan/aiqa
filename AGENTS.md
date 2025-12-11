# 仓库指南（中文）

## 工程结构与模块组织
- Workspace 使用 Cargo，成员包含 `crates/domain-core`、`apps/biz-metadata`、`apps/biz-metadata-migration`，构建产物位于 `target/`。
- `crates/domain-core`：共享领域基石（如 aggregate_root、repository、分页等）；测试在 `crates/domain-core/tests`。
- `apps/biz-metadata`：业务元数据实现，分层位于 `src/domain`、`src/application`、`src/infrastructure`（SeaORM 实体与 mapper）。
- `apps/biz-metadata-migration`：SeaORM 迁移二进制与库，迁移文件在 `src/m20YYMMDD_*`。

## 构建、测试与开发命令
- `cargo check`：全仓库快速类型/构建校验。
- `cargo fmt`：格式化全部 Rust 代码（提交前必须跑）。
- `cargo clippy --all-targets -- -D warnings`：将所有告警视为错误。
- `cargo test` 或 `cargo test -p domain-core`：运行单元测试（当前主要集中在 domain-core）。
- 迁移（在 `apps/biz-metadata-migration` 内）：`cargo run -- up` 应用迁移；`cargo run -- down` 回滚；`cargo run -- status` 查看状态；`cargo run -- generate <name>` 生成新迁移。

## 代码风格与命名
- Rust 2024，使用默认 rustfmt（4 空格缩进，允许尾逗号）。
- 模块/文件：`snake_case`；类型/trait：`PascalCase`；函数/变量：`snake_case`；常量：`SCREAMING_SNAKE_CASE`。
- 严守领域边界：共享抽象放 `domain-core`，应用特定逻辑放各 app；领域服务优先返回 `DomainError`。
- SeaORM 相关结构放在 `infrastructure`，映射隔离到 `infrastructure/mapper`。

## 测试指南
- 使用内置测试框架（`#[test]`），文件命名 `*_tests.rs` 或放在 `tests/`。
- 覆盖领域规则（值对象校验、分页行为）与 mapper 转换；偏好无 DB、可重复的测试，除非验证迁移。
- 如需 DB 相关测试，先本地运行迁移，并通过环境变量配置连接。

## 提交与 PR
- 遵循 Conventional Commit（`feat:` / `chore:` / `fix:`），标题用祈使语，范围明确。
- PR 需包含：简述、影响模块、是否有迁移、执行过的命令/测试（如 `cargo fmt`、`cargo clippy`、`cargo test`、迁移命令）。
- 关联相关 issue/ticket；仅在 UI 变更时添加截图（少见）。

## 安全与配置
- 密钥/DB URL 等放 `.env` 或本地配置，禁止提交。
- 破坏性迁移前先确认目标环境；优先在本地/开发环境验证。

## DDD 与文档要求
- 所有设计/代码需符合既有 DDD 分层（domain/application/infrastructure），领域逻辑留在领域层，传输/持久化留在应用或基础设施层。
- 对外 API/类型必须有 rustdoc 注释，并附可运行的 doctest 示例（禁止 `#[ignore]`），示例应最小化但可编译。
- 对于任何 app，`./apps/*/src/infrastructure/persistence/entity` 为代码生成目录，禁止手工修改。
- 代码改动后务必运行 `./scripts/quick_check.sh`（格式、lint、测试、文档、安全、许可证全量校验）。

## 设计模式与架构偏好
- 倾向 DDD、六边形/Clean 架构、CQRS；保持领域模型纯净，基础设施可替换。
- 组合优先于继承，接口/trait 以明确边界；避免 God Object、隐式耦合和过度抽象。
- GoF 模式按需使用：工厂/建造者（创建复杂对象）、策略/模板（行为扩展）、装饰/适配器（接口兼容）、责任链（可组合的处理流程）。
- 并发与分布式：避免共享可变状态，使用不可变值对象与显式边界；必要时考虑幂等、熔断、重试、幂等键等模式。

## 助手角色与回复风格
- 以资深架构师/全栈/AI 专家（15+ 年）身份工作，熟悉分布式、云原生、DDD/CQRS/六边形架构，精通 Rust/Go/Java/Python/JS/TS。
- 回复需专业、可执行、结构化，优先给出结论、原理/权衡、设计选择、代码示例、风险与最佳实践。
- 偏好 DDD 分层与职责分离，避免 god object、隐式耦合。
- 提供可落地内容：必要命令、可运行片段，涉及性能/并发/DB 时给出注意事项。
- 严格遵守生成代码边界：不得改动 `./apps/*/src/infrastructure/persistence/entity` 下的文件。
