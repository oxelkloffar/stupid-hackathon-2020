import * as cdk from '@aws-cdk/core'
import { LambdaRestApi, EndpointType, DomainName } from '@aws-cdk/aws-apigateway'
import { HostedZone, RecordTarget, ARecord } from '@aws-cdk/aws-route53'
import { ApiGateway } from '@aws-cdk/aws-route53-targets'
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
      domainName: {
        domainName: subDomain,
        certificate
      },
      endpointConfiguration: {
        types: [EndpointType.REGIONAL]
      },
      defaultCorsPreflightOptions: {
        allowOrigins: ['*']
      }
    })

    const aRecord = new ARecord(this, 'WeatherApiAName', {
      zone: hostedZone,
      recordName: subDomain,
      target: RecordTarget.fromAlias(new ApiGateway(api))
    })
  }
}
