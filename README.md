# rust-rocket-api

A sample [Rust](https://www.rust-lang.org/) app that uses [Rocket.rs](https://rocket.rs/) to implement a very basic API along with the manifest to deploy it to [Cloud Foundry](https://www.cloudfoundry.org/) using [a buildpack configured to use nightly](https://github.com/No9/rust-buildpack).

## pre-requisits
A cloud foundry account on a PaaS provider.
I've chosen [IBM Cloud](https://console.bluemix.net/registration/) as it has a no credit card free tier.

## usage

```
$ git clone https://github.com/No9/rust-rocket-api
$ cd rust-rocket-api
$ ibmcloud login -a https://api.eu-gb.bluemix.net
$ ibmcloud target --cf
$ ibmcloud cf push mywizzbang --random-route -p .
```