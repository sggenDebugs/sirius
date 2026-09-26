# Sirius
A real-time collaborative Kanban board written in Rust (Axum) and React. Deployed in an AWS EC2 environment.

## Features
- **Real-time Collaboration:** Instant updates across all connected clients using WebSockets.
- **High Performance:** Built with Rust and Axum for low-latency, high-concurrency handling.
- **Modern Frontend:** Responsive UI built with React.
- **Cloud-Native:** Containerized with ECR Docker and deployed on AWS EC2.

## Local Development
1. **Backend:** `cd sirius-backend && cargo run`
2. **Frontend:** `cd sirius-frontend && npm install && npm run dev`

## AWS EC2 Deployment Guide

To deploy Sirius to your AWS EC2 instance, follow these steps:

### 1. Prerequisites
- An active AWS Account with an EC2 instance running Ubuntu 22.04 LTS.
- Docker installed on both your local machine and the EC2 instance.
- AWS CLI configured locally with permissions for ECR and EC2.

### 2. Infrastructure Setup
Ensure your EC2 Security Group allows inbound traffic on the necessary ports:
- **Port 80 (TCP):** For the WebSocket and HTTP server.
- **Port 22 (TCP):** For SSH access (restrict to your IP for security).

### 3. Build and Push to Amazon ECR
Containerize the application and push it to the Elastic Container Registry (ECR).

```bash
# Execute below step in AWS Cloudshell:
# 1. Create the repository
aws ecr create-repository --repository-name sirius-app --region <region-name>

# Execute below steps In local machine
# 2. Authenticate Docker
aws ecr get-login-password --region <region-name> | docker login --username AWS --password-stdin <ACCOUNT_ID>.dkr.ecr.<region-name>.amazonaws.com

# 3. Build and Tag
docker build -t sirius-app .
docker tag sirius-app:latest <ACCOUNT_ID>.dkr.ecr.<region-name>.amazonaws.com/sirius-app:latest

# 4. Push to ECR
docker push <ACCOUNT_ID>.dkr.ecr.<region-name>.amazonaws.com/sirius-app:latest
```

### 4. Deploy on EC2
SSH into your EC2 instance and pull the image from ECR.

```bash
# 1. Authenticate on ec2 instance server
aws ecr get-login-password --region <region-name> | docker login --username AWS --password-stdin <ACCOUNT_ID>.dkr.ecr.<region-name>.amazonaws.com

# 2. Pull and Run
docker pull <ACCOUNT_ID>.dkr.ecr.<region-name>.amazonaws.com/sirius-app:latest
docker run -d -p 3000:3000 --name sirius-container <ACCOUNT_ID>.dkr.ecr.<region-name>.amazonaws.com/sirius-app:latest
```

### 5. Verification
Open your browser and navigate to `http://<EC2_PUBLIC_IP>`. You should see a successful connection message, indicating that your real-time Kanban board is live in the cloud.

## 📂 Project Structure
- `/sirius-backend`: Rust Axum API and WebSocket handlers.
- `/sirius-frontend`: React application for the UI.
- `Dockerfile`: Multi-stage build configuration for production.