import {interfaces} from 'network-interface';

const itfs = interfaces(IFF_ETH | IFF_RUNING);

console.log('network interfaces test done');

confole.log(interfaces());