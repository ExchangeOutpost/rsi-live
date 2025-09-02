# RSI Live Monitor

Real-time RSI (Relative Strength Index) monitoring with email notifications for ExchangeOutpost platform.

## Overview

This function continuously monitors the RSI indicator for a given financial symbol and sends email alerts when the RSI crosses configurable overbought or oversold thresholds. It's designed to help traders identify potential entry and exit points based on RSI momentum signals.

## Features

- **Real-time RSI Calculation**: Computes RSI using configurable periods (default: 14)
- **Email Notifications**: Automatic alerts when RSI crosses thresholds
- **Configurable Thresholds**: Customizable overbought/oversold levels
- **WebAssembly Deployment**: Runs efficiently on ExchangeOutpost platform

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `period` | integer | 14 | Number of periods for RSI calculation (1-200) |
| `limit_low` | number | 30.0 | Lower RSI threshold for oversold alerts (0-100) |
| `limit_high` | number | 70.0 | Upper RSI threshold for overbought alerts (0-100) |
| `email` | string | *required* | Email address to receive notifications |

## How It Works

1. **Data Input**: Receives candlestick data for the specified symbol
2. **RSI Calculation**: Computes RSI using the specified period
3. **Threshold Monitoring**: Compares current RSI against configured limits
4. **Alert Generation**: Sends email notifications when thresholds are breached:
   - **Oversold Alert**: When RSI drops below `limit_low`
   - **Overbought Alert**: When RSI rises above `limit_high`

## Example Usage

### Configuration
```json
{
  "period": 14,
  "limit_low": 25.0,
  "limit_high": 75.0,
  "email": "trader@example.com"
}
```

### Expected Behavior
- **RSI < 25**: Email sent with "RSI is below 25: [value] for symbol [SYMBOL]"
- **RSI > 75**: Email sent with "RSI is above 75: [value] for symbol [SYMBOL]"
- **25 ≤ RSI ≤ 75**: No alert sent

## RSI Indicator

The Relative Strength Index (RSI) is a momentum oscillator that:
- Ranges from 0 to 100
- Measures the speed and change of price movements
- Traditionally uses a 14-period calculation
- Common thresholds:
  - **Oversold**: RSI < 30 (potential buying opportunity)
  - **Overbought**: RSI > 70 (potential selling opportunity)

## Building

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM file will be available at:
`target/wasm32-unknown-unknown/release/rsi-live.wasm`

## Deployment

Deploy to ExchangeOutpost by uploading the compiled WASM file along with the `manifest.json` configuration.

## License

Licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.
