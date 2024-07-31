# --------------- #
# -- Version Args -- #
# --------------- #
ARG PYTHON_VERSION=3.12-slim
ARG WINE_MONO_VERSION=4.9.4
# --------------- #
# -- Python Development Tools -- #
# --------------- #
FROM python:${PYTHON_VERSION} AS python-tools

WORKDIR /app

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    binutils \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -r appuser && useradd -m -r -g appuser appuser \
    && chown -R appuser:appuser /home/appuser \
    && chown -R appuser:appuser /app

COPY ./Pipfile ./Pipfile.lock /app/

RUN python -m pip install pipenv black \
    && python -m pipenv --python "$(which python)" install -d --ignore-pipfile


# --------------- #
# -- Python Binary Build Stage -- #
# --------------- #
FROM python:${PYTHON_VERSION} AS python-build

WORKDIR /app

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    binutils \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -r appuser && useradd -m -r -g appuser appuser \
    && chown -R appuser:appuser /home/appuser \
    && chown -R appuser:appuser /app

USER appuser

ENV PATH="/home/appuser/.local/bin:${PATH}"

# Install pipenv and compile the Python application to a binary
COPY ./Pipfile ./Pipfile.lock /app/
RUN python -m pip install pipenv pyinstaller jinja2 \
    && python -m pipenv --python "$(which python)" install --deploy --ignore-pipfile

COPY . /app

RUN python -m pipenv run pyinstaller --onefile scripts/config.py


# --------------- #
# -- Steam CMD -- #
# --------------- #
FROM steamcmd/steamcmd:ubuntu

ARG WINEARCH=win64
ARG WINE_MONO_VERSION

ENV TZ=America/Los_Angeles
ENV PYTHONUNBUFFERED=1
ENV DISPLAY=:0
ENV PUID=1000
ENV PGID=1000
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Check who is 1000 and remove them if they arent steam
RUN if [ "$(getent passwd $PUID | cut -d: -f1)" != "steam" ]; then userdel $(getent passwd $PUID | cut -d: -f1); fi

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    apt-get update                        \
    && apt-get upgrade -y                 \
    && apt-get install -y -qq             \
        build-essential                   \
        htop net-tools nano gcc g++ gdb   \
        netcat-traditional curl wget zip unzip        \
        cron sudo gosu dos2unix  jq      \
        tzdata                            \
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

# Add 32-bit architecture


# Install Wine
ADD https://dl.winehq.org/wine-builds/winehq.key /tmp/winehq.key
RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    dpkg --add-architecture i386 \
    && apt-get update \
    && apt-get install -y software-properties-common gnupg2 \
    && apt-key add /tmp/winehq.key \
    && apt-add-repository 'deb https://dl.winehq.org/wine-builds/ubuntu/ bionic main' \
    && apt-get install -y --install-recommends \
    winehq-stable \
    winbind \
    cabextract \
    && rm -rf /var/lib/apt/lists/*

ENV WINEDEBUG=fixme-all

# Install Winetricks
ADD --chmod=755 https://raw.githubusercontent.com/Winetricks/winetricks/master/src/winetricks /usr/local/bin/winetricks

# Container informaiton
ARG GITHUB_SHA="not-set"
ARG GITHUB_REF="not-set"
ARG GITHUB_REPOSITORY="not-set"

ENV ENSHROUDED_CONFIG_DIR=/usr/local/share/enshrouded-config
ENV CONFIG_TEMPLATE_PATH=/home/steam/scripts/templates/config.json.j2

RUN usermod -u ${PUID} steam                                \
    && groupmod -g ${PGID} steam                            \
    && echo "steam ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers \
    && mkdir -p "${ENSHROUDED_CONFIG_DIR}"

USER steam

WORKDIR /home/steam

ENV HOME=/home/steam
ENV USER=steam
ENV LD_LIBRARY_PATH="/home/steam/.steam/sdk32:${LD_LIBRARY_PATH}"
ENV LD_LIBRARY_PATH="/home/steam/.steam/sdk64:${LD_LIBRARY_PATH}"
ENV PATH="/home/steam/.local/bin:${ENSHROUDED_CONFIG_DIR}:${PATH}"

# Setup a Wine prefix
ENV WINEPREFIX=/home/steam/.wine
ENV WINEARCH=${WINEARCH}
RUN winecfg

# Install Mono
ADD https://dl.winehq.org/wine/wine-mono/${WINE_MONO_VERSION}/wine-mono-${WINE_MONO_VERSION}.msi /mono/wine-mono-${WINE_MONO_VERSION}.msi
RUN wineboot -u && sudo msiexec /i /mono/wine-mono-${WINE_MONO_VERSION}.msi \
    && sudo rm -rf /mono/wine-mono-${WINE_MONO_VERSION}.msi \
    && mkdir -p "${HOME}/.local/bin" \
    && sudo rm -rf /var/lib/apt/lists/* \
    && sudo chown -R steam:steam "${ENSHROUDED_CONFIG_DIR}"

COPY --from=python-build --chmod=steam:$PGID /app/dist/ "${ENSHROUDED_CONFIG_DIR}"

RUN sudo chown -R steam:steam /home/steam

COPY --chown=${PUID}:${PGID} ./scripts /home/steam/scripts

EXPOSE 15636 15637

RUN echo "source /home/steam/scripts/utils.sh" >> /home/steam/.bashrc

ENTRYPOINT ["/bin/bash","/home/steam/scripts/entrypoint.sh"]
