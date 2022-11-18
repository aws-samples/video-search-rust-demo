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
import { AccessPoint, FileSystem, PerformanceMode } from "aws-cdk-lib/aws-efs";
import { IVpc } from "aws-cdk-lib/aws-ec2";
import { RemovalPolicy } from "aws-cdk-lib";

export interface TantivyIndexStorageProps {
  readonly vpc: IVpc;
}

export class TantivyIndexStorage extends Construct {
  public readonly fileSystem: FileSystem;
  public readonly accessPoint: AccessPoint;

  constructor(scope: Construct, id: string, props: TantivyIndexStorageProps) {
    super(scope, id);

    const { vpc } = props;

    this.fileSystem = new FileSystem(this, "TantivyStorage", {
      vpc,
      performanceMode: PerformanceMode.MAX_IO,
      removalPolicy: RemovalPolicy.DESTROY,
    });

    this.accessPoint = this.fileSystem.addAccessPoint("RootPoint", {
      path: "/tantivy",
      createAcl: {
        ownerUid: "1001",
        ownerGid: "1001",
        permissions: "750",
      },
      posixUser: {
        gid: "1001",
        uid: "1001",
      },
    });
  }
}
