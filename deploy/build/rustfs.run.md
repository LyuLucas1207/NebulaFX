# NebulaFX Service Installation Guide

## 1. Prerequisites

### 1.1 Create System User

```bash
# Create nebulafx system user and group without login shell
sudo useradd -r -s /sbin/nologin nebulafx
```

### 1.2 Create Required Directories

```bash
# Create program directory
sudo mkdir -p /opt/nebulafx

# Create data directories
sudo mkdir -p /data/nebulafx/{vol1,vol2}

# Create configuration directory
sudo mkdir -p /etc/nebulafx

# Set directory permissions
sudo chown -R nebulafx:nebulafx /opt/nebulafx /data/nebulafx
sudo chmod 755 /opt/nebulafx /data/nebulafx
```

## 2. Install NebulaFX

```bash
# Copy NebulaFX binary
sudo cp nebulafx /usr/local/bin/
sudo chmod +x /usr/local/bin/nebulafx

# Copy configuration file
sudo cp obs.yaml /etc/nebulafx/
sudo chown -R nebulafx:nebulafx /etc/nebulafx
```

## 3. Configure Systemd Service

```bash
# Copy service unit file
sudo cp nebulafx.service /etc/systemd/system/

# Reload systemd configuration
sudo systemctl daemon-reload
```

## 4. Service Management

### 4.1 Start Service

```bash
sudo systemctl start nebulafx
```

### 4.2 Check Service Status

```bash
sudo systemctl status nebulafx
```

### 4.3 Enable Auto-start

```bash
sudo systemctl enable nebulafx
```

### 4.4 View Service Logs

```bash
# View real-time logs
sudo journalctl -u nebulafx -f

# View today's logs
sudo journalctl -u nebulafx --since today
```

## 5. Verify Installation

```bash
# Check service ports
ss -tunlp | grep 9000

# Test service availability
curl -I http://localhost:9000
```
