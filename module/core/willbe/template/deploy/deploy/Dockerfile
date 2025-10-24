FROM google/cloud-sdk
ENV TF_VERSION=1.7.4

WORKDIR /

# Install terraform
RUN apt update --allow-releaseinfo-change \
    && apt install wget unzip \
    && mkdir -p /usr/lib/terraform/${TF_VERSION} \
    && cd /usr/lib/terraform/${TF_VERSION} \
    && wget https://releases.hashicorp.com/terraform/${TF_VERSION}/terraform_${TF_VERSION}_linux_amd64.zip \
    && unzip terraform_${TF_VERSION}_linux_amd64.zip \
    && chmod 755 /usr/lib/terraform/${TF_VERSION}/terraform \
    && ln -s /usr/lib/terraform/${TF_VERSION}/terraform /usr/bin/terraform

WORKDIR /app

ARG google_sa_creds
ENV GOOGLE_APPLICATION_CREDENTIALS /app/$google_sa_creds

VOLUME /var/run/docker.sock

CMD [ "make", "deploy-in-container" ]
