# DeFi Risk Watcher Agent

An AI-powered agent for monitoring DeFi protocol risks, analyzing on-chain data, and providing real-time risk assessments.

## Features

- **Protocol Risk Analysis**: Automated risk scoring for DeFi protocols
- **On-Chain Monitoring**: Real-time blockchain data analysis
- **TVL Tracking**: Total Value Locked monitoring and alerts
- **Smart Contract Analysis**: Audit report parsing and risk assessment
- **Market Risk Detection**: Volatility, liquidity, and correlation analysis
- **Alert System**: Configurable risk threshold notifications
- **Historical Tracking**: Store and analyze risk trends over time

## Quick Start

```bash
# Set API keys
export OPENAI_API_KEY="your-openai-key"
export ETHERSCAN_API_KEY="your-etherscan-key"
export ALCHEMY_API_KEY="your-alchemy-key"

# Analyze a protocol
wadah run wadah.yaml --prompt "Analyze the risk profile of Uniswap V3"

# Interactive monitoring
wadah run wadah.yaml --interactive
```

## Example Queries

### Protocol Analysis

```bash
# Risk assessment
wadah run wadah.yaml --prompt "What are the current risks for Aave protocol?"

# TVL changes
wadah run wadah.yaml --prompt "Show TVL changes for major DeFi protocols in the last 24h"

# Smart contract check
wadah run wadah.yaml --prompt "Check the audit status and security score for 0x..."
```

### Market Monitoring

```bash
# Volatility analysis
wadah run wadah.yaml --prompt "Analyze volatility risks for ETH-USDC LP positions"

# Liquidation risk
wadah run wadah.yaml --prompt "Calculate liquidation risks for current positions"

# Correlation analysis
wadah run wadah.yaml --prompt "What's the correlation between BTC and ETH today?"
```

### Alert Configuration

```bash
# Set monitoring rules
wadah run wadah.yaml --prompt "Alert me if Aave TVL drops by more than 10%"

# Price alerts
wadah run wadah.yaml --prompt "Notify if ETH price falls below $2000"
```

## Risk Metrics

The agent analyzes multiple risk dimensions:

### Smart Contract Risk
- Audit status and findings
- Code complexity
- Upgrade mechanisms
- Admin key controls

### Financial Risk
- TVL concentration
- Liquidity depth
- Slippage analysis
- Impermanent loss potential

### Market Risk
- Price volatility
- Correlation with major assets
- Market cap stability

### Operational Risk
- Team transparency
- Governance structure
- Oracle dependencies
- Cross-chain bridge risks

## Data Sources

- **On-Chain**: Ethereum, Arbitrum, Optimism, Polygon (via Alchemy/Infura)
- **Pricing**: CoinGecko API
- **DeFi Metrics**: DefiLlama API
- **Contract Data**: Etherscan API
- **Audit Reports**: Public audit databases

## Configuration

### Add Protocols

Edit `data/protocols.json`:
```json
{
  "protocols": [
    {
      "name": "Uniswap V3",
      "addresses": ["0x..."],
      "risk_threshold": 50
    }
  ]
}
```

### Adjust Sensitivity

Edit `wadah.yaml` budgets and thresholds.

### Custom Alerts

Modify alert rules in ToolCaps.json rate limits.

## Safety & Security

### Read-Only Operations

- **No transaction signing**: Agent cannot execute trades
- **No wallet access**: Monitoring only, no fund control
- **API rate limits**: Prevents abuse and excessive costs

### Data Privacy

- Risk data stored locally in `./data/`
- No sensitive keys in traces
- Optional encryption for stored data

## Output & Reports

### Generate Risk Report

```bash
wadah run wadah.yaml --prompt "Generate a comprehensive risk report for top 10 DeFi protocols" \
  --trace reports/risk-report-$(date +%Y%m%d).jsonl
```

### Export Alerts

```bash
# View trace with alerts
wadah trace stats reports/risk-report-20250101.jsonl
```

## Use Cases

- **Fund Managers**: Portfolio risk monitoring
- **Protocol Teams**: Competitor analysis
- **Researchers**: DeFi market studies
- **Traders**: Risk-adjusted strategy planning
- **Auditors**: Automated preliminary risk assessment

## Architecture

```
Risk Query → Agent → On-Chain Data APIs
                ↓
        Risk Analysis Engine
                ↓
     Historical Comparison + ML
                ↓
     Risk Score + Recommendations
```

## Advanced Features

### Continuous Monitoring

Run as a background service:
```bash
# TODO: Implement daemon mode
wadah run wadah.yaml --daemon --alert-webhook https://your-webhook
```

### Multi-Chain Support

Configure additional chains in `tools/web3.yaml`.

### Custom Risk Models

Extend risk scoring in `code/risk_models.py`.

## Contributing

Add new data sources, risk metrics, or alert types via the templates system.

## License

Apache 2.0

