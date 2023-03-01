# GSB API

GSB REST API allows to bind and unbind GSB services.

Binding GSB service enables WebSocket endpoint which allows to listen on incoming GSB messages.
Path to WebSocket endpoint is returned in bind service response.

API documentation:

- REST endpoints OpenAPI [schema](../specs/gsb-api.yaml).

- WS AsyncAPI [schema](../specs/gsb-api-messages.yaml).

  Open it directly or by using AsyncAPI Studio.

  `docker run -it -p 8000:80 asyncapi/studio`

## Sample GSB API usage scenarios

### Basic scenario

```mermaid
sequenceDiagram
    participant App as WebSocket Client
    participant Backend as Backend Yagna
    participant Service as Golem Service

    App ->>+ Backend: POST /gsb-api/v1/services:<br> { listen: { on: '/public/gftp/myapp', <br> components: ['GetMetadata', 'GetChunk'] }}
    Backend -->> App: HTTP 201<br>{ listen: { ..., links: { messages: <br>'gsb-api/v1/services/<br>L3B1YmxpYy9nZnRwL215YXBw' } } }
    App ->> Backend: GET gsb-api/v1/services/<br>L3B1YmxpYy9nZnRwL215YXBw <br> Headers: { Upgrade: websocket }
    Backend -->> App: HTTP 101

    par WebSocket connection lifespan
        App ->> Backend: WS Event: Open
        loop Multiple GSB message calls
            Service ->>+ Backend: GSB: GetChunk <br>public/gftp/myapp
            Backend ->> App: WS Message: gsbRequest<br> { id: '<br>L3B1YmxpYy9nZnRwL215YXBw', <br>component: 'GetMetadata', <br>payload: { .. } }
            App -->> Backend: WS Message: gsbResponse<br> { id: '<br>L3B1YmxpYy9nZnRwL215YXBw', payload: { .. } }
            Backend -->>- Service : GSB: GftpMetadata
        end
        App ->> Backend: DELETE /gsb-api/v1/services/<br>L3B1YmxpYy9nZnRwL215YXBw
        Backend -->> App: WS Event: Close<br> CloseCode: Normal
    end

    Backend -->>- App: HTTP 200
```

### Early GSB request scenario (simplified)

```mermaid
sequenceDiagram
    participant App as WebSocket Client
    participant Backend as Backend Yagna
    participant Service as Golem Service

    App ->>+ Backend: POST /gsb-api/v1/services
    Backend -->> App: HTTP 201
    Service ->>+ Backend: GSB: GetChunk
    Note right of Backend: Buffered GSB request
    App ->> Backend: GET gsb-api/v1/services/<br>L3B1YmxpYy9nZnRwL215YXBw
    Backend -->> App: HTTP 101
    par WebSocket connection lifespan
        App ->> Backend: WS Event: Open
        Backend ->> App: WS Message: gsbRequest
        App -->> Backend: WS Message: gsbResponse
        Backend -->>- Service : GSB: GftpMetadata
        App ->> Backend: DELETE /gsb-api/v1/services/<br>L3B1YmxpYy9nZnRwL215YXBw
        Backend -->> App: WS Event: Close<br> CloseCode: Normal
    end
    Backend -->>- App: HTTP 200
```

#### GSB messages buffering and cancelling (simplified)

```mermaid
sequenceDiagram
    participant App as WebSocket Client
    participant Backend as Backend Yagna
    participant Service as Golem Service

    App ->>+ Backend: POST /gsb-api/v1/services
    Backend -->> App: HTTP 201
    App ->> Backend: GET gsb-api/v1/services/<br>L3B1YmxpYy9nZnRwL215YXBw
    Backend -->> App: HTTP 101

    par 1st WS connection lifespan
        App -->> Backend: WS Event: Close<br>CloseCode: Abnormal
    end

    Service ->>+ Backend: GSB: GetMetadata
    Note right of Backend: Buffering GSB request<br>when WS disconnected
    App ->> Backend: GET gsb-api/v1/services/<br>L3B1YmxpYy9nZnRwL215YXBw
    Backend -->> App: HTTP 101

    par 2nd WS connection lifespan
        App ->> Backend: WS Event: Open
        Backend ->> App: WS Message: gsbRequest
        App -->> Backend: WS Message: gsbResponse
        Backend -->>- Service : GSB: GftpMetadata
        Service ->>+ Backend: GSB: GetChunk
        Backend ->> App: WS Message: gsbRequest
        App -->> Backend: WS Event: Close<br>CloseCode: Abnormal
    end

    Note right of Backend: Canceling GSB request<br> on WS Close
    Backend -->>- Service: GSB: Error<br>ya_service_bus::Error::Closed
    App ->> Backend: DELETE /gsb-api/v1/services/<br>L3B1YmxpYy9nZnRwL215YXBw
    Backend -->>- App: HTTP 200
```

### Disconnect WS on new WS connection (simplified)

```mermaid
sequenceDiagram
    participant App0 as WebSocket Client<br>Old session
    participant App1 as WebSocket Client<br>New session
    participant Backend as Backend Yagna

    App0 ->>+ Backend: POST /gsb-api/v1/services
    Backend -->> App0: HTTP 201
    App0 ->> Backend: GET gsb-api/v1/services/<br>L3B1YmxpYy9nZnRwL215YXBw
    Backend -->> App0: HTTP 101

    par Old WS connection
        App0 ->> Backend: WS Event: Open
        App1 ->> Backend: GET gsb-api/v1/services/<br>L3B1YmxpYy9nZnRwL215YXBw
        Backend -->> App0: WS Event: Close<br>CloseCode: Policy
    end
    par New WS connection
    Backend -->> App1: HTTP 101

    App1 ->> Backend: WS Event: Open
    App1 ->> Backend: DELETE /gsb-api/v1/services/<br>L3B1YmxpYy9nZnRwL215YXBw
    Backend -->> App1: WS Event: Close<br> CloseCode: Normal
    end

    Backend -->>- App0: HTTP 200
```
