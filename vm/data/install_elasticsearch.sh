#!/bin/sh

# Update package lists
apt-get update

# Install JRE (Java Runtime Environment)
apt-get install -y default-jre

# Install Elasticsearch from the .deb package in /vdata
dpkg -i /vagrant_data/elasticsearch-8.17.4-amd64.deb || true
apt-get install -f -y

# Configure Elasticsearch to bind to all interfaces
sed -i 's/#network.host: 192.168.0.1/network.host: 0.0.0.0/' /etc/elasticsearch/elasticsearch.yml


# Disable security features for simplicity (not recommended for production)
sed -i 's/xpack.security.enabled: true/xpack.security.enabled: false/' /etc/elasticsearch/elasticsearch.yml
# echo "xpack.security.enabled: false" >> /etc/elasticsearch/elasticsearch.yml

# Enable and start Elasticsearch service
systemctl daemon-reload
systemctl enable elasticsearch.service
systemctl start elasticsearch.service

echo "Elasticsearch installation completed. It may take a minute to start up."
echo "You can access it at http://localhost:9200 from your host machine."