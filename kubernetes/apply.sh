    kubectl apply -f dockerhub-secret.yaml
    kubectl apply -f postgres-configmap.yaml
    kubectl apply -f postgres-deployment.yaml
    kubectl apply -f postgres-persistance.yaml
    kubectl apply -f postgres-persistance-claim.yaml
    kubectl apply -f postgres-secret.yaml
    kubectl apply -f postgres-service.yaml
    kubectl apply -f webapp.yaml