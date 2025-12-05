FROM mcr.microsoft.com/devcontainers/dotnet:1-9.0

RUN apt-get update -y && \
  apt-get upgrade -y && \
  apt-get install -y hunspell hunspell-en-gb && \
  apt-get clean && \ 
  rm -rf /var/lib/apt/lists/*