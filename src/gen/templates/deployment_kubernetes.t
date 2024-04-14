to: "kubernetes/{{pkg_name}}.yaml"
skip_exists: true
message: "Kubernetes file generated successfully."
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{pkg_name}}
spec:
  minReplicas: 3
  selector:
    matchLabels:
      app: {{pkg_name}}
  template:
    metadata:
      labels:
        app: {{pkg_name}}
    spec:
      containers:
      - name: {{pkg_name}}
        image: {{pkg_name}}
        resources:
          requests:
            cpu: 10m
            memory: 10M
          limits:
            cpu: 20m
            memory: 20M
        env:
          - name: DATABASE_URL
            value: postgres://postgres:admin@postgres/url_mapper_prod?sslmode=disable

---
apiVersion: v1
kind: Service
metadata:
  name: {{pkg_name}}
spec:
  selector:
    app: {{pkg_name}}
  ports:
  - port: 3000
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: {{pkg_name}}
  namespace: {{pkg_name}}
spec:
  rules:
    - http:
        paths:
          - pathType: Prefix
            path: "/"
            backend:
              service:
                name: {{pkg_name}}
                port:
                  number: 3000
---
apiVersion: v1
kind: ServiceAccount
metadata:
  name: secret-creator
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: secret-creator-role
rules:
- apiGroups: [""]
  resources: ["secrets"]
  verbs: ["create", "delete", "get", "list"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: secret-creator-binding
subjects:
- kind: ServiceAccount
  name: secret-creator
roleRef:
  kind: Role
  name: secret-creator-role
  apiGroup: rbac.authorization.k8s.io
---
apiVersion: batch/v1
kind: Job
metadata:
  name: jwt-secret-generator
spec:
  template:
    spec:
      serviceAccountName: secret-creator
      restartPolicy: Never
      containers:
      - name: jwt-secret-creator
        image: bitnami/kubectl
        command: ["/bin/sh", "-c"]
        args:
          - |
            #!/bin/sh
            set -e
            # Generate a random JWT secret key
            JWT_SECRET=$(openssl rand -base64 32)
            # Create or update the secret in Kubernetes
            kubectl create secret generic jwt-secret --from-literal=key="$JWT_SECRET" -o yaml --dry-run=client | kubectl apply -f -
            # Output the generated JWT secret key (remove this line in production to avoid leaking secrets)
            echo "Generated JWT Secret: $JWT_SECRET"
      restartPolicy: OnFailure
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: {{pkg_name}}-config
data:
    production.yaml: |
        logger:
          enable: true
          pretty_backtrace: true
          level: error
          format: compact
        server:
          port: 3000
          host: http://localhost
          middlewares:
            etag:
              enable: true
            limit_payload:
              enable: true
              body_limit: 5mb
            logger:
              enable: false
            catch_panic:
              enable: true
            timeout_request:
              enable: false
              timeout: 5000
            cors:
              enable: true
              # Set the value of the [`Access-Control-Allow-Origin`][mdn] header
              # allow_origins:
              #   - https://loco.rs
              # Set the value of the [`Access-Control-Allow-Headers`][mdn] header
              # allow_headers:
              # - Content-Type
              # Set the value of the [`Access-Control-Allow-Methods`][mdn] header
              # allow_methods:
              #   - POST
              # Set the value of the [`Access-Control-Max-Age`][mdn] header in seconds
              # max_age: 3600
            static:
              enable: true
              must_exist: true
              precompressed: false
              folder:
                uri: "/"
                path: "frontend/dist"
              fallback: "frontend/dist/index.html"
        workers:
          mode: BackgroundQueue
        mailer:
          smtp:
            enable: true
            host: {{ get_env(name="MAILER_HOST", default="localhost") }}
            port: 1025
            secure: false

        # Initializers Configuration
        # initializers:
        #  oauth2:
        #    authorization_code: # Authorization code grant type
        #      - client_identifier: google # Identifier for the OAuth2 provider. Replace 'google' with your provider's name if different, must be unique within the oauth2 config.
        #        ... other fields

        database:
          uri: {{get_env(name="DATABASE_URL", default="postgres://loco:loco@localhost:5432/loco_app")}}
          enable_logging: false
          connect_timeout: 500
          idle_timeout: 500
          min_connections: 1
          max_connections: 10
          auto_migrate: true
          dangerously_truncate: false
          dangerously_recreate: false

        redis:
          uri: {{get_env(name="REDIS_URL", default="redis://127.0.0.1")}}
          dangerously_flush: false

        auth:
          jwt:
            secret: {{get_env(name="AUTH_JWT_SECRET", default="PqRwLF2rhHe8J22oBeHy")}}
            expiration: 604800