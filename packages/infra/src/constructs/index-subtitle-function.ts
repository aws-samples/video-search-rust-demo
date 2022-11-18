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
import { IVpc } from "aws-cdk-lib/aws-ec2";
import { RustLambdaFunction } from "./rust-lambda-function";
import { Architecture, Code, FileSystem } from "aws-cdk-lib/aws-lambda";
import { Duration } from "aws-cdk-lib";
import { IAccessPoint } from "aws-cdk-lib/aws-efs";
import { ITopic } from "aws-cdk-lib/aws-sns";
import { SnsEventSource } from "aws-cdk-lib/aws-lambda-event-sources";

export interface IndexSubtitleFunctionProps {
  readonly vpc: IVpc;
  readonly tantivyAccessPoint: IAccessPoint;
  readonly subtitleResultTopic: ITopic;
}

export class IndexSubtitleFunction extends Construct {
  public readonly rustFunction: RustLambdaFunction;
  constructor(scope: Construct, id: string, props: IndexSubtitleFunctionProps) {
    super(scope, id);

    const { vpc, tantivyAccessPoint, subtitleResultTopic } = props;

    const mountPath = "/mnt/tantivy";

    this.rustFunction = new RustLambdaFunction(this, "Function", {
      vpc,
      code: Code.fromAsset("../lambda/.dist/index_subtitle/"),
      architecture: Architecture.ARM_64,
      environment: {
        TANTIVY_MOUNT: mountPath,
      },
      timeout: Duration.seconds(300),
      memorySize: 512,
      filesystem: FileSystem.fromEfsAccessPoint(tantivyAccessPoint, mountPath),
    });

    this.rustFunction.func.addEventSource(
      new SnsEventSource(subtitleResultTopic)
    );
  }
}
