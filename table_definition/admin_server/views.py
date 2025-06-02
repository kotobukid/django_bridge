"""
Django Bridge - ヘルスチェック機能

Axumプロジェクトから一時起動されたDjango開発サーバーの
生存確認用エンドポイントを提供します。
"""
from django.http import HttpResponse, JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.views.decorators.http import require_http_methods
import json


@csrf_exempt
@require_http_methods(["GET", "POST"])
def health_check_admin(request):
    """
    ヘルスチェックエンドポイント
    
    Axumサーバーからの生存確認リクエストに応答します。
    GETとPOSTの両方をサポートしています。
    """
    if request.method == 'GET':
        return HttpResponse('OK', content_type='text/plain')
    
    elif request.method == 'POST':
        # POST時はより詳細な情報を返す
        response_data = {
            'status': 'ok',
            'service': 'django-bridge-admin',
            'timestamp': request.META.get('HTTP_DATE', ''),
        }
        return JsonResponse(response_data)
