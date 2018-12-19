# upnp-honeypot
Honeypot emulating vulnerable UPNP enabled home network gateway

## Repository structure:
- soapserver/
  - Server responsible for the UPNP SOAP interface (**Unimplemented**)
- ssdpserver/
  - Server that advertises the SOAP server using SSDP (Valid responses are returned although requests are not yet fully parsed)

## Resources:
- [UPNP spec](http://upnp.org/specs/arch/UPnP-arch-DeviceArchitecture-v1.1.pdf)
- [Overview of UPNP](http://www.upnp-hacks.org/upnp.html)
- [Rapid7 report on UPNP vulnerabilities](https://information.rapid7.com/rs/411-NAK-970/images/SecurityFlawsUPnP%20(1).pdf) (Primarily implimentation specific memory safety problems)
- [Overview of Internet Gateway Device vulnerabilities](http://www.upnp-hacks.org/igd.html)
- [Akamai report on attackers using UPNP to create proxy networks](https://www.akamai.com/us/en/multimedia/documents/white-paper/upnproxy-blackhat-proxies-via-nat-injections-white-paper.pdf)
