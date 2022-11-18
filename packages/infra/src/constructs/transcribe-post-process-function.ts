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

import { Duration } from "aws-cdk-lib";
import { ITable } from "aws-cdk-lib/aws-dynamodb";
import { IVpc } from "aws-cdk-lib/aws-ec2";
import { Rule } from "aws-cdk-lib/aws-events";
import { LambdaFunction } from "aws-cdk-lib/aws-events-targets";
import { Architecture, Code } from "aws-cdk-lib/aws-lambda";
import { IQueue } from "aws-cdk-lib/aws-sqs";
import { Construct } from "constructs";
import { RustLambdaFunction } from "./rust-lambda-function";

export interface TranscribePostProcessFunctionProps {
  readonly vpc: IVpc;
  readonly dynamoDbTable: ITable;
  readonly subtitleJobQueue: IQueue;
}

export class TranscribePostProcessFunction extends Construct {
  public readonly rustFunction: RustLambdaFunction;
  public readonly transcribeJobCompletedRule: Rule;

  constructor(
    scope: Construct,
    id: string,
    props: TranscribePostProcessFunctionProps
  ) {
    super(scope, id);

    const { vpc, dynamoDbTable, subtitleJobQueue } = props;

    this.rustFunction = new RustLambdaFunction(this, "Function", {
      vpc,
      code: Code.fromAsset("../lambda/.dist/transcribe_post_process/"),
      architecture: Architecture.ARM_64,
      environment: {
        DYNAMODB_TABLE_NAME: dynamoDbTable.tableName,
        QUEUE_URL: subtitleJobQueue.queueUrl,
      },
      timeout: Duration.seconds(15),
    });
    dynamoDbTable.grantReadWriteData(this.rustFunction.func);
    subtitleJobQueue.grantSendMessages(this.rustFunction.func);

    this.transcribeJobCompletedRule = new Rule(
      this,
      "TranscribeJobCompletedRule",
      {
        eventPattern: {
          source: ["aws.transcribe"],
          detailType: ["Transcribe Job State Change"],
          detail: {
            TranscriptionJobStatus: ["COMPLETED"],
          },
        },
      }
    );
    this.transcribeJobCompletedRule.addTarget(
      new LambdaFunction(this.rustFunction.func)
    );
  }
}
