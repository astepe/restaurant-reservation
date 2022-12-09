import boto3


def lambda_handler(event, context):
    instance_id = event["instance_id"]
    if event["action"] == "hibernate":
        boto3.client("ec2").stop_instances(InstanceIds=[instance_id], Hibernate=True)
    return instance_id