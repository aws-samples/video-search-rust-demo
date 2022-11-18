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
import {
  Architecture,
  Code,
  Runtime,
  Function,
  ILayerVersion,
} from "aws-cdk-lib/aws-lambda";
import { Duration } from "aws-cdk-lib";
import { IVpc } from "aws-cdk-lib/aws-ec2";
import { FileSystem } from "aws-cdk-lib/aws-lambda";

export interface RustLambdaFunctionProps {
  functionName?: string;
  code: Code;
  architecture: Architecture;
  vpc?: IVpc;
  timeout?: Duration;
  readonly environment?: {
    [key: string]: string;
  };
  filesystem?: FileSystem;
  memorySize?: number;
  layers?: ILayerVersion[];
}

export class RustLambdaFunction extends Construct {
  public readonly func: Function;

  constructor(scope: Construct, id: string, props: RustLambdaFunctionProps) {
    super(scope, id);

    this.func = new Function(this, `${id}Function`, {
      runtime: Runtime.PROVIDED_AL2,
      handler: "bootstrap",
      ...props,
    });
  }
}
