import * as cdk from '@aws-cdk/core'
import { LambdaRestApi, EndpointType, DomainName } from '@aws-cdk/aws-apigateway'
import { HostedZone, CnameRecord } from '@aws-cdk/aws-route53'
import { DnsValidatedCertificate, ValidationMethod } from '@aws-cdk/aws-certificatemanager'
import { Function, Runtime, Code } from '@aws-cdk/aws-lambda'
import * as path from 'path'

export class WeatherApiStack extends cdk.Stack {

  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, {
      env: {
        account: process.env.AWS_ACCOUNT_ID,
        region: process.env.AWS_REGION
      },
      ...props
    })

    const domainName = process.env.DNS_DOMAIN || 'aourell.se'
    const hostedZone = HostedZone.fromLookup(this, 'WeatherApiDomain', {
      domainName
    })

    const dnsPrefix = process.env.DNS_PREFIX || 'weatherstack-proxy-api'
    const subDomain = `${dnsPrefix}.${domainName}`
    const certificate = new DnsValidatedCertificate(this, 'WeatherApiCert', {
      domainName: subDomain,
      hostedZone: hostedZone,
      validationMethod: ValidationMethod.DNS
    })

    const handler = new Function(this, 'WeatherApiLambda', {
      runtime: Runtime.PROVIDED,
      code: Code.fromAsset(path.normalize('../rust-weatherstack-proxy-lambda/target/lambda/release/bootstrap.zip')),
      handler: 'doesnt.matter',
      environment: {
      }
    })

    const api = new LambdaRestApi(this, 'WeatherApiSpec', {
      handler,
      parameters: {
        'country': 'string'
      },
      endpointConfiguration: {
        types: [EndpointType.REGIONAL]
      },
      defaultCorsPreflightOptions: {
        allowOrigins: ['*']
      }
    })

    const domain = new DomainName(this, 'WeatherApiSubDomain', {
      domainName: subDomain,
      certificate,
      endpointType: EndpointType.REGIONAL
    })
    
    domain.addBasePathMapping(api, { basePath: 'v1'})
  }
}
