FROM mcr.microsoft.com/dotnet/sdk:10.0

RUN rm -f /etc/apt/sources.list.d/yarn.list \
  && apt-get update -y \
  && apt-get install -y --no-install-recommends cloc hunspell hunspell-en-gb \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*