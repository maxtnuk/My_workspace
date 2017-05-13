package com.example.maxtnt.test_android;

import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.net.Uri;
import android.os.Bundle;
import android.provider.Telephony;
import android.support.v7.app.AppCompatActivity;
import android.telephony.SmsMessage;
import android.view.LayoutInflater;
import android.view.View;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.TextView;
import android.widget.Toast;

public class MainActivity extends AppCompatActivity {

    public static final int REQUEST_CODE_ANOTHER =1001;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
    }
    public class SMSActivity extends BroadcastReceiver {
        @Override
        public void onReceive(final Context context, final Intent intent) {
            if (intent.getAction().equals("android.provider.Telephony.SMS_RECEIVED")){
                LinearLayout sms_print = (LinearLayout) findViewById(R.id.smsrespond);

                LayoutInflater inflater= (LayoutInflater) getSystemService(Context.LAYOUT_INFLATER_SERVICE);
                inflater.inflate(R.layout.sms_respond,sms_print,true);

                TextView title = (TextView) findViewById(R.id.title);
                TextView content= (TextView) findViewById(R.id.content);
                Button see_it = (Button) findViewById(R.id.opensms);

                for (SmsMessage smsMessage : Telephony.Sms.Intents.getMessagesFromIntent(intent)) {
                    content.setText(smsMessage.getMessageBody());
                    title.setText(smsMessage.getMessageClass().name());
                }
                see_it.setOnClickListener(new View.OnClickListener() {
                    @Override
                    public void onClick(View view) {
                        Intent intent = new Intent(Intent.ACTION_VIEW);
                        intent.setFlags(Intent.FLAG_ACTIVITY_CLEAR_TOP);
                        startActivity(intent);
                    }
                });

            }

        }
    }
    public void press_one_click(View v){
        Toast.makeText(getApplicationContext(),"you pressed right now",Toast.LENGTH_LONG).show();
    }
    public void naver_into(View v){
        Intent naver_access = new Intent(Intent.ACTION_VIEW, Uri.parse("http://m.naver.com"));
        startActivity(naver_access);
    }
    public void on_click_new(View v){
        Intent next =new Intent(getApplicationContext(),NextActivity.class);
        startActivityForResult(next,REQUEST_CODE_ANOTHER);
    }
    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);

        if (requestCode == REQUEST_CODE_ANOTHER){
            Toast.makeText(getBaseContext(),"onActivity result 요청코드 "+ requestCode+", 결과 코드: "+resultCode,Toast.LENGTH_LONG).show();
            if(resultCode==RESULT_OK){
                Toast.makeText(getBaseContext(),"onActivity result 요청코드 "+ data.getExtras().getString("name"),Toast.LENGTH_LONG).show();
            }
        }
    }
}