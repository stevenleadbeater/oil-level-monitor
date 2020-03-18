FROM maven:ibmjava-alpine
WORKDIR /opt/liquibase
COPY tables ./tables
COPY db.changelog.xml .
COPY pom.xml .
RUN mvn dependency:resolve
CMD mvn process-resources