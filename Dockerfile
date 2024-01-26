# --------------- #
# -- Steam CMD -- #
# --------------- #
FROM steamcmd/steamcmd:ubuntu

ENV TZ=America/Los_Angeles
ENV PYTHONUNBUFFERED=1
ENV DISPLAY=:0
ENV DISPLAY=:1
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update                        \
    && apt-get upgrade -y                 \
    && apt-get install -y -qq             \
        build-essential                   \
        htop net-tools nano gcc g++ gdb   \
        netcat curl wget zip unzip        \
        cron sudo gosu dos2unix  jq       \
        tzdata python3 python3-pip        \
        lib32z1 lib32gcc-s1 lib32stdc++6  \
    && rm -rf /var/lib/apt/lists/*        \
    && gosu nobody true                   \
    && dos2unix

RUN addgroup --system steam     \
    && adduser --system         \
      --home /home/steam        \
      --shell /bin/bash         \
      steam                     \
    && usermod -aG steam steam  \
    && chmod ugo+rw /tmp/dumps

# Install wget
RUN apt-get update
RUN apt-get install -y wget

# Add 32-bit architecture
RUN dpkg --add-architecture i386
RUN apt-get update

# Install Wine
RUN apt-get install -y software-properties-common gnupg2
RUN wget -nc https://dl.winehq.org/wine-builds/winehq.key
RUN apt-key add winehq.key
RUN apt-add-repository 'deb https://dl.winehq.org/wine-builds/ubuntu/ bionic main'
RUN apt-get install -y --install-recommends winehq-stable winbind
ENV WINEDEBUG=fixme-all

# Setup a Wine prefix
ENV WINEPREFIX=/root/.demo
ENV WINEARCH=win64
RUN winecfg

# Install Mono
RUN wget -P /mono http://dl.winehq.org/wine/wine-mono/4.9.4/wine-mono-4.9.4.msi
RUN wineboot -u && msiexec /i /mono/wine-mono-4.9.4.msi
RUN rm -rf /mono/wine-mono-4.9.4.msi

# Install Winetricks
RUN apt-get install -y cabextract
RUN wget https://raw.githubusercontent.com/Winetricks/winetricks/master/src/winetricks
RUN chmod +x winetricks
RUN cp winetricks /usr/local/bin

# Install Xvfb
RUN apt-get install -y xvfb

# Install Visual C++ Redistributable
RUN #wineboot -u && xvfb-run winetricks vcrun2022

# Container informaiton
ARG GITHUB_SHA="not-set"
ARG GITHUB_REF="not-set"
ARG GITHUB_REPOSITORY="not-set"

ENV PUID=1000
ENV PGID=1000

RUN usermod -u ${PUID} steam                                \
    && groupmod -g ${PGID} steam                            \
    && echo "steam ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

USER steam

WORKDIR /home/steam

ENV HOME=/home/steam
ENV USER=steam
ENV LD_LIBRARY_PATH="/home/steam/.steam/sdk32:${LD_LIBRARY_PATH}"
ENV LD_LIBRARY_PATH="/home/steam/.steam/sdk64:${LD_LIBRARY_PATH}"
ENV PATH="/home/steam/.local/bin:${PATH}"


# Setup a Wine prefix
ENV WINEPREFIX=/home/steam/.wine
ENV WINEARCH=win64
RUN winecfg

# Install .NET Framework 4.5.2
RUN wineboot -u && winetricks -q dotnet45

COPY --chown=${PUID}:${PGID} ./Pipfile ./Pipfile.lock /home/steam/scripts/

RUN pip3 install pipenv \
    && cd /home/steam/scripts \
    && pipenv install --system --deploy --ignore-pipfile \
    && sudo chown -R steam:steam /home/steam

COPY --chown=${PUID}:${PGID} ./scripts /home/steam/scripts

EXPOSE 15636 15637
EXPOSE 27015/udp

RUN echo "source /home/steam/scripts/utils.sh" >> /home/steam/.bashrc

#HEALTHCHECK --interval=1m --timeout=3s \
#    CMD pidof valheim_server.x86_64 || exit 1

ENTRYPOINT ["/bin/bash","/home/steam/scripts/entrypoint.sh"]
