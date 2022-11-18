/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: MIT-0
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this
 * software and associated documentation files (the "Software"), to deal in the Software
 * without restriction, including without limitation the rights to use, copy, modify,
 * merge, publish, distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
 * INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

import { Construct } from "constructs";
import { RustLambdaFunction } from "./rust-lambda-function";
import { Architecture, Code, IFunction } from "aws-cdk-lib/aws-lambda";
import { Duration } from "aws-cdk-lib";
import { IVpc } from "aws-cdk-lib/aws-ec2";
import { ITable } from "aws-cdk-lib/aws-dynamodb";
import { IQueue } from "aws-cdk-lib/aws-sqs";
import { IDistribution } from "aws-cdk-lib/aws-cloudfront";

export interface AppFunctionProps {
  readonly vpc: IVpc;
  readonly dynamoDbTable: ITable;
  readonly subtitleJobQueue: IQueue;
  readonly searchSubtitleFunction: IFunction;
  readonly distribution: IDistribution;
}

export class AppFunction extends Construct {
  public readonly rustFunction: RustLambdaFunction;
  constructor(scope: Construct, id: string, props: AppFunctionProps) {
    super(scope, id);

    const {
      vpc,
      dynamoDbTable,
      subtitleJobQueue,
      searchSubtitleFunction,
      distribution,
    } = props;

    this.rustFunction = new RustLambdaFunction(this, "Function", {
      vpc,
      code: Code.fromAsset("../app/.dist/app/"),
      architecture: Architecture.ARM_64,
      timeout: Duration.seconds(5),
      environment: {
        CONTENT_HOST: distribution.distributionDomainName,
        DYNAMODB_TABLE_NAME: dynamoDbTable.tableName,
        TANTIVY_SEARCH_FUNCTION_NAME: searchSubtitleFunction.functionName,
        SUBTITLE_QUEUE_URL: subtitleJobQueue.queueUrl,
      },
    });
    dynamoDbTable.grantReadWriteData(this.rustFunction.func);
    subtitleJobQueue.grantSendMessages(this.rustFunction.func);
    searchSubtitleFunction.grantInvoke(this.rustFunction.func);
  }
}
