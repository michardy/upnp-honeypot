# upnp-honeypot
Honeypot emulating vulnerable UPNP enabled home network gateway

## Repository structure:
- soapserver/
	- Server responsible for the UPNP SOAP interface (**Unimplemented**)
		- Expose XML device descriptions
		- Expose RPC control endpoints
- ssdpserver/
	- UDP Server that advertises the SOAP server using SSDP (Valid responses are returned although requests are not yet fully parsed)
		- Respond to M-Search requests (Todo)
		- Index all SSDP fields into Elasticsearch (In progress)
		- Block frequent repeat requests. (Done)

The UPNP event notification system will not be implimented at this time.
When it is it will be nessisary to add a UDP server for event transmission and possibly subscription
(The UPNP spec is a little vague as to whether subscriptions requests are sent over HTTP or UDP).

## Resources:
- [UPNP spec](http://upnp.org/specs/arch/UPnP-arch-DeviceArchitecture-v1.1.pdf)
- [Overview of UPNP](http://www.upnp-hacks.org/upnp.html)
- [Rapid7 report on UPNP vulnerabilities](https://information.rapid7.com/rs/411-NAK-970/images/SecurityFlawsUPnP%20(1).pdf) (Primarily implimentation specific memory safety problems)
- [Overview of Internet Gateway Device vulnerabilities](http://www.upnp-hacks.org/igd.html)
- [Akamai report on attackers using UPNP to create proxy networks](https://www.akamai.com/us/en/multimedia/documents/white-paper/upnproxy-blackhat-proxies-via-nat-injections-white-paper.pdf)
