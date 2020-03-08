# Stupid Hackathon Sweden 2020

## What is this?
This is a mono-repo for all our projects/ideas for the Stupid Hackathon Sweden 2020... and we apparently had time for 1 project, in the end.

## Weather App
In the `country-picker` directory, you can find our awesome contribution: *an app that tells you the weather*... given you can provide the desired location, of course :)

When you open the (web) app, you are asked to type in the population count of the country for which you want to know the current weather.

### Demo!
https://oxelkloffar.github.io/stupid-hackathon-2020

### Run it (the frontend) locally
```
country-picker> npm run dev
```

### Build it (everything) yourself

#### Frontend
```
country-picker> npm run build
country-picker> npm run deploy
```
Currently, the npm deploy script always uses our github pages page for deployment, edit that script if you want to deploy somewhere else.

#### Backend
```
country-picker/api/rust-weatherstack-proxy-lambda> ./build.sh <weatherstack api key>
country-picker/api/.aws-cdk> npm run build
country-picker/api/.aws-cdk> cdk deploy
```
This deploys to AWS, so you need to provide the following env vars:
* `AWS_ACCOUNT_ID`: numerical aws account id
* `AWS_REGION`: the aws region you want to deploy to
* `DNS_DOMAIN`: domain name (defauklt: "aourell.se")
* `DNS_PREFIX`: sub-domain to use (default: "weatherstack-proxy-api")
