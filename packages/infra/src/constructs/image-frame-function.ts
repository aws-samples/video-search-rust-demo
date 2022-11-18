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
import { IVpc } from "aws-cdk-lib/aws-ec2";
import {
  Architecture,
  Code,
  LayerVersion,
  Runtime,
} from "aws-cdk-lib/aws-lambda";
import { IBucket } from "aws-cdk-lib/aws-s3";
import { Construct } from "constructs";
import { RustLambdaFunction } from "./rust-lambda-function";

export interface ImageFrameFunctionProps {
  vpc: IVpc;
  bucket: IBucket;
}

export class ImageFrameFunction extends Construct {
  public readonly ffmpegLayer: LayerVersion;
  public readonly rustFunction: RustLambdaFunction;

  constructor(scope: Construct, id: string, props: ImageFrameFunctionProps) {
    super(scope, id);

    const { vpc, bucket } = props;

    this.ffmpegLayer = new LayerVersion(this, "FfmpegLayer", {
      compatibleRuntimes: [Runtime.PROVIDED_AL2],
      code: Code.fromAsset("./layers/ffmpeg/"),
      description: "arm64 Static FFMpeg binary layer",
    });

    this.rustFunction = new RustLambdaFunction(this, "Function", {
      vpc,
      code: Code.fromAsset("../lambda/.dist/image_frame/"),
      architecture: Architecture.ARM_64,
      timeout: Duration.seconds(60),
      layers: [this.ffmpegLayer],
      environment: {
        BUCKET_NAME: bucket.bucketName,
      },
    });
    bucket.grantReadWrite(this.rustFunction.func);
  }
}
