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

import { Stack, StackProps } from "aws-cdk-lib";
import {BuildSpec, ComputeType, LinuxBuildImage} from "aws-cdk-lib/aws-codebuild";
import { PDKPipeline } from "aws-prototyping-sdk/pipeline";
import { Construct } from "constructs";

export class PipelineStack extends Stack {
  readonly pipeline: PDKPipeline;

  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

    this.pipeline = new PDKPipeline(this, "ApplicationPipeline", {
      primarySynthDirectory: "packages/infra/cdk.out",
      defaultBranchName: "main",
      repositoryName: this.node.tryGetContext("repositoryName") || "monorepo",
      publishAssetsInParallel: false,
      crossAccountKeys: true,
      sonarCodeScannerConfig: this.node.tryGetContext("sonarqubeScannerConfig"),
      synthCodeBuildDefaults: {
        buildEnvironment: {
          buildImage: LinuxBuildImage.STANDARD_6_0,
          computeType: ComputeType.X2_LARGE,
        },
        partialBuildSpec: BuildSpec.fromObject({
          version: "0.2",
          phases: {
            install: {
              "commands": [
                "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
                ". $HOME/.cargo/env",
                "pip3 install cargo-lambda",
              ]
            }
          }
        })
      },
      synth: {}
    });

  }
}
