# Video Search

This repo is a sample video search app using AWS services.

## Features

- Transcribing Video and generate subtitle.
- Translate subtitle and generate subtitle.
- Search subtitle and jump to selected part of the video.

## Screenshots
![List](docs/list.png)
![Detail En](docs/detail_en.png)
![Detail Ko](docs/deatil_ko.png)
![Search En](docs/search_en.png)
![Search Ko](docs/search_ko.png)

## Architecture

### Transcribe
![Transcribe](docs/transcribe.drawio.svg)

### Generate Subtitle (VTT)
![Generate Subtitle](docs/subtitle.drawio.svg)

### Code Pipeline
![Code Pipeline](docs/code-pipeline.drawio.svg)

### Demo App
![Demo App](docs/app.drawio.svg)

## CDK

```bash
# install cdk
$ npm i aws-cdk

# bootstrap cdk
$ cdk bootstrap

# if you want to use aws credential profile
$ cdk bootstrap --profile yourprofile
```

## Deploy

```bash
# build lambda package.
$ nx run lambda:build

# on packages/infra
$ npm install

# aws credential setup required.
$ cdk deploy PipelineStack

# After deploying the stack, then the CodeCommit repo will be created.
# If you want to deploy this app, you have to push this repo to the CodeCommit repo.
$ git remote add codecommit <CodeCommit-Repo>
$ git push codecommit main
```

## Usage
Just upload video file to s3 that is generated by CDK. Then the video will appear on Demo app.
Video file s3 key must follow the following structure.

```
video/myvideo.en-US.mp4
```
- "video" is s3 key prefix for upload to s3. Only the video that include `video` prefix will be triggered by lambda.
- "myvideo" will be the title.
- "en-US" is language code that used by transcribe. Refer this [link](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html). 
- "mp4" is the file's extention. currently only mp4 and mov are supported.

You can access on demo app through the endpoint of deployed api gateway.

## Search Engine
This sample use [tantivy](https://github.com/quickwit-oss/tantivy) for searching subtitle.
In particular, Among non-Latin languages, Korean is only supported (I used [this](https://github.com/lindera-morphology/lindera-tantivy)).

## Security

See [CONTRIBUTING](CONTRIBUTING.md#security-issue-notifications) for more information.

## License

This library is licensed under the MIT-0 License. See the LICENSE file.

