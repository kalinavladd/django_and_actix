from django.contrib.sessions.models import Session
from rest_framework import mixins, viewsets
from rest_framework.permissions import IsAuthenticated

from api.blog.serializers import PostSerializer
from blog.models import Post


class ProjectApiViewSet(mixins.RetrieveModelMixin,
                        mixins.ListModelMixin,
                        mixins.CreateModelMixin,
                        mixins.DestroyModelMixin,
                        viewsets.GenericViewSet):
    serializer_class = PostSerializer
    permission_classes = [IsAuthenticated]

    def get_queryset(self):
        return Post.objects.filter(author=self.request.user)
