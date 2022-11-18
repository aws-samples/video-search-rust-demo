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
import { Construct } from "constructs";
import { MediaStorage } from "./constructs/media-storage";
import { MediaVpc } from "./constructs/media-vpc";
import { MediaDynamodb } from "./constructs/media-dynamodb";
import { TranscribeFunction } from "./constructs/transcribe-function";
import { TranscribePostProcessFunction } from "./constructs/transcribe-post-process-function";
import { SubtitleJobQueue } from "./constructs/subtitle-job-queue";
import { SubtitleFunction } from "./constructs/subtitle-function";
import { Topic } from "aws-cdk-lib/aws-sns";
import { IndexSubtitleFunction } from "./constructs/index-subtitle-function";
import { TantivyIndexStorage } from "./constructs/tantivy-index-storage";
import { SearchSubtitleFunction } from "./constructs/search-subtitle-function";
import { AppFunction } from "./constructs/app-function";
import { AppApiGateway } from "./constructs/app-api-gateway";
import { ImageFrameFunction } from "./constructs/image-frame-function";

export class ApplicationStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const { vpc } = new MediaVpc(this, "VPC");

    const mediaStorage = new MediaStorage(this, "Bucket");

    const mediaDynamodb = new MediaDynamodb(this, "MediaDB");

    const imageFrameFunction = new ImageFrameFunction(
      this,
      "ImageFrameFunction",
      {
        vpc,
        bucket: mediaStorage.bucket,
      }
    );

    new TranscribeFunction(this, "TranscribeFunction", {
      vpc,
      dynamoDbTable: mediaDynamodb.table,
      eventSourceBucket: mediaStorage.bucket,
      imageFrameFunction: imageFrameFunction.rustFunction.func,
    });

    const subtitleJobQueue = new SubtitleJobQueue(this, "SubtitleJobQueue");

    new TranscribePostProcessFunction(this, "TranscribePostProcessFunction", {
      vpc,
      dynamoDbTable: mediaDynamodb.table,
      subtitleJobQueue: subtitleJobQueue.queue,
    });

    const subtitleResultTopic = new Topic(this, "SubtitleResultTopic");

    new SubtitleFunction(this, "SubtitleFunction", {
      vpc,
      dynamoDbTable: mediaDynamodb.table,
      subtitleJobQueue: subtitleJobQueue.queue,
      mediaSourceBucket: mediaStorage.bucket,
      subtitleResultTopic,
    });

    const tantivyIndexStorage = new TantivyIndexStorage(
      this,
      "TantivyIndexStorage",
      {
        vpc,
      }
    );

    new IndexSubtitleFunction(this, "IndexSubtitleFunction", {
      vpc,
      tantivyAccessPoint: tantivyIndexStorage.accessPoint,
      subtitleResultTopic,
    });

    const searchSubtitleFunction = new SearchSubtitleFunction(
      this,
      "SearchSubtitleFunction",
      {
        vpc,
        tantivyAccessPoint: tantivyIndexStorage.accessPoint,
      }
    );

    const appFunction = new AppFunction(this, "AppFunction", {
      vpc,
      subtitleJobQueue: subtitleJobQueue.queue,
      searchSubtitleFunction: searchSubtitleFunction.rustFunction.func,
      dynamoDbTable: mediaDynamodb.table,
      distribution: mediaStorage.distribution,
    });

    new AppApiGateway(this, "AppApiGateway", {
      handler: appFunction.rustFunction.func,
    });
  }
}
