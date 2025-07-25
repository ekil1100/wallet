# Rust Web3 CLI Wallet

一个轻量级的以太坊命令行钱包，使用 Rust 和 ethers-rs 构建。

## 功能特性

- 🔐 生成新钱包（私钥和地址）
- 📥 从私钥导入钱包
- 💰 查询地址余额
- 📤 发送 ETH 交易
- 💾 钱包文件保存和加载

## 安装

```bash
# 克隆项目
git clone <your-repo>
cd wallet

# 构建项目
cargo build --release

# 运行
cargo run -- --help
```

## 配置

1. 复制环境配置文件：
```bash
cp .env.example .env
```

2. 编辑 `.env` 文件，设置你的 RPC URL。你可以使用以下免费选项：

**选项 1：使用公共 RPC（可能有速率限制）**
```
ETH_RPC_URL=https://eth.public-rpc.com
```

**选项 2：获取免费的 API Key（推荐）**
- [Alchemy](https://www.alchemy.com/) - 注册获取免费 API Key
- [Infura](https://infura.io/) - 注册获取免费 API Key
- [QuickNode](https://www.quicknode.com/) - 注册获取免费 API Key

然后设置：
```
ETH_RPC_URL=https://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY
```

## 使用方法

### 生成新钱包
```bash
# 生成并显示新钱包
cargo run -- new

# 生成并保存到文件
cargo run -- new --save wallet.json
```

### 导入钱包
```bash
# 从私钥导入
cargo run -- import --key YOUR_PRIVATE_KEY

# 导入并保存
cargo run -- import --key YOUR_PRIVATE_KEY --save wallet.json
```

### 查看地址
```bash
cargo run -- address --wallet wallet.json
```

### 查询余额
```bash
# 查询指定地址余额
cargo run -- balance --address 0x742d35Cc6634C0532925a3b844Bc9e7595f8fA49

# 查询钱包文件中的地址余额
cargo run -- balance --wallet wallet.json
```

### 发送交易
```bash
cargo run -- send --to 0x742d35Cc6634C0532925a3b844Bc9e7595f8fA49 --amount 0.01 --wallet wallet.json
```

## 安全提示

- ⚠️ 私钥非常重要，请妥善保管
- 🔒 不要在公共电脑上使用此钱包
- 📝 建议将私钥备份在安全的地方
- 🚫 不要将包含私钥的文件提交到版本控制系统

## 网络支持

默认连接到以太坊主网。可以通过修改 RPC URL 来连接到其他网络：
- Mainnet: `https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY`
- Goerli: `https://eth-goerli.g.alchemy.com/v2/YOUR_KEY`
- Sepolia: `https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY`

## License

MIT