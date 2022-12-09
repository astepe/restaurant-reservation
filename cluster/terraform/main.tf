provider "aws" {
  region = "us-east-1"
}

data "aws_iam_policy_document" "lambda_assume_role_policy" {
  statement {
    effect  = "Allow"
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "lambda_role" {
  name               = "hibernate_microk8s-restaurant-reservation_lambda_role"
  assume_role_policy = data.aws_iam_policy_document.lambda_assume_role_policy.json
  inline_policy {
    name = "allow-lambda-to-start-and-hibernate-ec2"
    policy = jsonencode(
      {
        "Version" : "2012-10-17",
        "Statement" : [
          {
            "Effect" : "Allow",
            "Action" : [
              "logs:CreateLogGroup",
              "logs:CreateLogStream",
              "logs:PutLogEvents"
            ],
            "Resource" : "arn:aws:logs:*:*:*"
          },
          {
            "Effect" : "Allow",
            "Action" : [
              "ec2:Start*",
              "ec2:Stop*"
            ],
            "Resource" : "arn:aws:ec2:*:*:instance/i-0e7c2148295f48e75"
          }
        ]
      }
    )
  }
}

resource "aws_lambda_function" "start_hibernate_lambda_function" {
  filename      = "start_hibernate_microk8s-restaurant-reservation.zip"
  function_name = "start_hibernate_microk8s-restaurant-reservation"
  description   = "Starts or hibernates the microk8s-restaurant-reservation EC2 instance"
  handler       = "lambda_function.lambda_handler"
  runtime       = "python3.9"
  role          = aws_iam_role.lambda_role.arn
  timeout       = "10"
}

resource "aws_cloudwatch_event_rule" "hibernate_instance" {
  name                = "microk8s-restaurant-reservation_hibernate_rule"
  schedule_expression = "cron(0 22 ? * 1-7 *)"
}

resource "aws_cloudwatch_event_target" "hibernate_instance" {
  arn  = aws_lambda_function.start_hibernate_lambda_function.arn
  rule = aws_cloudwatch_event_rule.hibernate_instance.name
  input = <<EOF
{
  "instance_id": "i-0e7c2148295f48e75",
  "action": "hibernate"
}
EOF
}

resource "aws_lambda_permission" "allow_start_or_hibernate_event_to_invoke_lambda" {
  statement_id  = "AllowInvoke${aws_lambda_function.start_hibernate_lambda_function.function_name}FromEventBridge"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.start_hibernate_lambda_function.function_name
  principal     = "events.amazonaws.com"
  source_arn    = "arn:aws:events:us-east-1:488458563198:rule/microk8s-restaurant-reservation*"
}

data "archive_file" "python_lambda_package" {
  type        = "zip"
  source_file = "lambda_function.py"
  output_path = "start_hibernate_microk8s-restaurant-reservation.zip"
}
