# Cushion

Cushion is Solana implementation of Curve's [crvUSD](https://github.com/curvefi/curve-stablecoin) and lending protocol (documented [here](https://github.com/curvefi/curve-stablecoin/blob/master/doc/curve-stablecoin.pdf)).

Collateral Dept Positions (CDP) protocols emitting stablecoins often have steep liquidation fees. The goal to give good incentives for liquidator to get rid of the bad protocol debt. Also, liquidations are done on full positions or chunks of them which can impact require more sophisticated liquidators.

Cushion addresses these issues by using Automated Market Makers (AMM) to provide traders incentives to rebalance the debt and collateral. AMMs are not Constant Product AMM like UniV2 but instead use a cubic curve. AMMs are distributed over a wide price range in "bands". The size of the bands is controlled by the amplification parameter

## Design

###
