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

import { RemovalPolicy } from "aws-cdk-lib";
import {
  FlowLogDestination,
  FlowLogTrafficType,
  SubnetType,
  Vpc,
} from "aws-cdk-lib/aws-ec2";
import { LogGroup, RetentionDays } from "aws-cdk-lib/aws-logs";
import { Construct } from "constructs";

export class MediaVpc extends Construct {
  public readonly vpc: Vpc;

  constructor(scope: Construct, id: string) {
    super(scope, id);

    const cloudWatchLogs = new LogGroup(this, "VpcLogGroup", {
      logGroupName: "/aws/vpc/flowlogs/media",
      retention: RetentionDays.ONE_MONTH,
      removalPolicy: RemovalPolicy.DESTROY,
    });

    this.vpc = new Vpc(this, "MediaVpc", {
      vpcName: "media",
      subnetConfiguration: [
        {
          cidrMask: 24,
          name: "application",
          subnetType: SubnetType.PRIVATE_WITH_EGRESS,
        },
        {
          cidrMask: 24,
          name: "ingress",
          subnetType: SubnetType.PUBLIC,
        },
      ],
      flowLogs: {
        cloudwatch: {
          destination: FlowLogDestination.toCloudWatchLogs(cloudWatchLogs),
          trafficType: FlowLogTrafficType.ALL,
        },
      },
    });
  }
}
