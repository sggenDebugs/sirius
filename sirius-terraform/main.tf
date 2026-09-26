provider "aws" {
  region = "ap-southeast-2"
}

resource "aws_vpc" "app_vpc" {
    cidr_block = "10.0.0.0/16"
    enable_dns_support = true
    enable_dns_hostnames = true
    tags = {
      Name = "sirius-vpc"
    }
}

resource "aws_subnet" "private_subnet_1a" {
  vpc_id            = aws_vpc.app_vpc.id
  cidr_block        = "10.0.1.0/24"
  availability_zone = "ap-southeast-2"
  tags              = { Name = "sirius-private-1a" }
}

resource "aws_subnet" "private_subnet_1b" {
  vpc_id            = aws_vpc.app_vpc.id
  cidr_block        = "10.0.2.0/24"
  availability_zone = "ap-southeast-2"
  tags              = { Name = "sirius-private-1b" }
}