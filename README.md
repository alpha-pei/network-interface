# `@yat/network-interface`

![https://github.com/alpha-pei/network-interface/actions](https://github.com/alpha-pei/network-interface/workflows/CI/badge.svg)

> `network-interface` is a node module witch impled by rust. It can list your deivce's network interfaces. 


# Usage

```ts
import {interfaces} from '@yat/network-interface';

interfaces(IFF_ETH | IFF_RUNING);

```
## Install this test package

```bash
yarn add @yat/network-interface
```

### Constants
```ts
//Filter Running interfaces
const IFF_ETH: number
// Filter Ethernet interfaces.
const IFF_LOOPBACK: number
// Filter Wireless interfaces sometimes it sames as Etherent interfaces.
const IFF_RUNNING: number
// Filter out VPN interfaces. Note! This is only a hypothesis.
const IFF_TUNN: number
///Filter out TUN interfaces. Note! This is only a hypothesis.
const IFF_VPN: number
///Filter out LOOPBACK interfaces.
const IFF_WIRELESS: number
```
