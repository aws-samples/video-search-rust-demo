import { CdkGraph, FilterPreset, Filters } from "@aws/pdk/cdk-graph";
import { CdkGraphDiagramPlugin } from "@aws/pdk/cdk-graph-plugin-diagram";
import { CdkGraphThreatComposerPlugin } from "@aws/pdk/cdk-graph-plugin-threat-composer";
import { AwsPrototypingChecks, PDKNag } from "@aws/pdk/pdk-nag";
import { PipelineStack } from "./stacks/pipeline-stack";
import { ApplicationStage } from "./stacks/application-stage";

/* eslint-disable @typescript-eslint/no-floating-promises */
(async () => {
  const app = PDKNag.app({
    nagPacks: [new AwsPrototypingChecks()],
  });

  const pipelineStack = new PipelineStack(app, "PipelineStack", {
    env: {
      account: process.env.CDK_DEFAULT_ACCOUNT!,
      region: process.env.CDK_DEFAULT_REGION!,
    },
  });

  const devStage = new ApplicationStage(app, "Dev", {
    env: {
      account: process.env.CDK_DEFAULT_ACCOUNT!, // Replace with Dev account
      region: process.env.CDK_DEFAULT_REGION!, // Replace with Dev region
    },
  });
  
  pipelineStack.pipeline.addStage(devStage);

  const graph = new CdkGraph(app, {
    plugins: [
      new CdkGraphDiagramPlugin({
        defaults: {
          filterPlan: {
            preset: FilterPreset.COMPACT,
            filters: [{ store: Filters.pruneCustomResources() }],
          },
        },
      }),
      new CdkGraphThreatComposerPlugin(),
    ],
  });

  app.synth();
  await graph.report();
})();
