#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from '@aws-cdk/core';
import { WeatherApiStack } from '../lib/weather-api-stack';

const app = new cdk.App();
new WeatherApiStack(app, 'WeatherApiStack');
