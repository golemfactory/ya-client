## Yagna public API
Public Yagna REST APIs client binding with Data Model and specifications in [OpenAPI](http://spec.openapis.org/) format.

### Some of GSB API scenarios

#### Basic scenario

```mermaid
sequenceDiagram
    participant App as "WebSocket Client"
    participant Backend as "Backend Yagna"
    participant Service as "Golem Service"
    
    App ->>+ Backend: POST /gsb-api/v1/services:<br> { listen: { on: '/public/gftp/myapp', <br> components: ['GetMetadata', 'GetChunk'] }}
    
    Backend -->> App: HTTP 201<br>{ listen: { ..., links: { messages: <br>'gsb-api/v1/services/bXlhcHAK' } } }

    App ->> Backend: GET gsb-api/v1/services/bXlhcHAK <br> Headers: { Upgrade: websocket }

    Backend -->> App: HTTP 101

    par WebSocket connection lifespan

        App ->> Backend: WS Event: Open

        loop Multiple GSB message calls
        
            Service ->>+ Backend: GSB: GetChunk <br>public/gftp/myapp

            Backend ->> App: WS Message: gsbRequest<br> { id: 'bXlhcHAK', <br>component: 'GetMetadata', <br>payload: { .. } }

            App -->> Backend: WS Message: gsbResponse<br> { id: 'bXlhcHAK', payload: { .. } }
            
            Backend -->>- Service : GSB: GftpMetadata

        end
        
        App ->> Backend: DELETE /gsb-api/v1/services/bXlhcHAK

        Backend -->> App: WS Event: Close<br> CloseCode: Normal

    end
    
    Backend -->>- App: HTTP 200
```

#### Early GSB request scenario

```mermaid
sequenceDiagram
    participant App as "WebSocket Client"
    participant Backend as "Backend Yagna"
    participant Service as "Golem Service"
    App ->>+ Backend: POST /gsb-api/v1/services
    Backend -->> App: HTTP 201
    Service ->>+ Backend: GSB: GetChunk
    Note right of Backend: Buffered GSB request
    App ->> Backend: GET gsb-api/v1/services/bXlhcHAK
    Backend -->> App: HTTP 101
    par WebSocket connection lifespan
        App ->> Backend: WS Event: Open
        Backend ->> App: WS Message: gsbRequest
        App -->> Backend: WS Message: gsbResponse
        Backend -->>- Service : GSB: GftpMetadata
        App ->> Backend: DELETE /gsb-api/v1/services/bXlhcHAK
        Backend -->> App: WS Event: Close<br> CloseCode: Normal
    end
    Backend -->>- App: HTTP 200
```

#### Some error scenarios (simplified)

```mermaid
sequenceDiagram
    participant App as "WebSocket Client"
    participant Backend as "Backend Yagna"
    participant Service as "Golem Service"
    App ->>+ Backend: POST /gsb-api/v1/services
    Backend -->> App: HTTP 201
    App ->> Backend: GET gsb-api/v1/services/bXlhcHAK
    Backend -->> App: HTTP 101
    par 1st WS connection lifespan
        App -->> Backend: WS Event: Close<br>CloseCode: Abnormal
    end
    Service ->>+ Backend: GSB: GetMetadata
    Note right of Backend: Buffered GSB request
    App ->> Backend: GET gsb-api/v1/services/bXlhcHAK
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
    App ->> Backend: DELETE /gsb-api/v1/services/bXlhcHAK
    Backend -->>- App: HTTP 200
```
