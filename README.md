# stdin-to-cloudwatch

Send logs from stdin to AWS CloudWatch Logs

## Usage

```sh
echo "Hello, World!" | stdin-to-cloudwatch MyLogGroup MyLogStream
```

- `MyLogGroup` is the name of the log group to send the logs to
  - If the log group does not exist, it will be created
- `MyLogStream` is the name of the log stream to send the logs to
  - If the log stream does not exist, it will be created
- The logs are read from stdin, and sent to CloudWatch Logs each line at a time

## Installation

```sh
cargo install stdin-to-cloudwatch
```

## Options

### `--region`

The AWS region to send the logs to.

```sh
echo "Hello, World!" | stdin-to-cloudwatch MyLogGroup MyLogStream --region us-west-2
```

### `--endpoint-url`

The endpoint URL to send the logs to.  
LocalStack users can use this to send logs to LocalStack.

```sh
echo "Hello, World!" | stdin-to-cloudwatch MyLogGroup MyLogStream --endpoint-url http://localhost:4566
```

### `--remake-log-stream`

If the log stream already exists, delete it and create a new one.

```sh
echo "Hello, World!" | stdin-to-cloudwatch MyLogGroup MyLogStream --remake-log-stream
```
