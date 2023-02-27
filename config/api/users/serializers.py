from rest_framework import serializers


class BaseUserSerializer(serializers.Serializer):

    id = serializers.IntegerField(read_only=True)
    first_name = serializers.CharField(allow_blank=True)
    last_name = serializers.CharField(allow_blank=True)
    is_staff = serializers.BooleanField()


class LoginSerializer(serializers.Serializer):
    username = serializers.CharField()
    password = serializers.CharField()
