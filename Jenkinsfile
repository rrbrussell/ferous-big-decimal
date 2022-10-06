void setBuildStatus(String message, String state) {
  step([
      $class: "GitHubCommitStatusSetter",
      reposSource: [$class: "ManuallyEnteredRepositorySource", url: env.GIT_URL],
      contextSource: [$class: "ManuallyEnteredCommitContextSource", context: "ci/jenkins/build-status"],
      errorHandlers: [[$class: "ChangingBuildStatusErrorHandler", result: "UNSTABLE"]],
      statusResultSource: [ $class: "ConditionalStatusResultSource", results: [[$class: "AnyBuildResult", message: message, state: state]] ]
  ]);
}

pipeline {
    agent any
    stages {
        stage('Mark Pending') {
            steps {
                setBuildStatus("Build Pending", "PENDING");
            }
        }
        stage('Build') {
            steps {
                sh 'cargo build'
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test'
            }
        }

        stage('Build Documentation') {
            steps {
                sh 'cargo doc --workspace --document-private-items'
            }
        }

        stage('Create Artifacts') {
            steps {
                sh "tar -cvzf ferrous-big-decimal-${env.BUILD_NUMBER}.tar.gz ./src/ Cargo.toml LICENSE README.md"
                sh "tar -cvzf ferrous-big-gecimal-docs-${env.BUILD_NUMBER}.tar.gz -C ./target/doc ./"
                archiveArtifacts artifacts: "ferrous-big-decimal-${env.BUILD_NUMBER}.tar.gz", fingerprint: true
                archiveArtifacts artifacts: "ferrous-big-decimal-docs-${env.BUILD_NUMBER}.tar.gz", fingerprint: true
            }
        }
    }
    post {
        success {
            setBuildStatus("Build succeeded", "SUCCESS");
        }
        failure {
            setBuildStatus("Build failed", "FAILURE");
        }
    }
}
