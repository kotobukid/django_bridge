"""
URL configuration for table_definition project.

The `urlpatterns` list routes URLs to views. For more information please see:
    https://docs.djangoproject.com/en/5.1/topics/http/urls/
Examples:
Function views
    1. Add an import:  from my_app import views
    2. Add a URL to urlpatterns:  path('', views.home, name='home')
Class-based views
    1. Add an import:  from other_app.views import Home
    2. Add a URL to urlpatterns:  path('', Home.as_view(), name='home')
Including another URLconf
    1. Import the include() function: from django.urls import include, path
    2. Add a URL to urlpatterns:  path('blog/', include('blog.urls'))
"""
from django.contrib import admin
from django.urls import path, re_path
from django.conf import settings
from django.views.static import serve

from admin_server.views import health_check_admin

urlpatterns = [
    # path('admin/', admin.site.urls),
    path('admin_proxy/health-check', health_check_admin),
    path(f'{settings.CUSTOM_ADMIN_ROOT}health-check', health_check_admin),
    path(settings.CUSTOM_ADMIN_ROOT, admin.site.urls),
]

if not settings.DEBUG:
    urlpatterns += [
        re_path(r'^%s(?P<path>.*)$' % settings.STATIC_URL.lstrip('/'), serve, {'document_root': settings.STATIC_ROOT}),
    ]
