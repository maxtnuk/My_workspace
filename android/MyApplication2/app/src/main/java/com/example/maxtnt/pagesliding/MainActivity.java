package com.example.maxtnt.pagesliding;

import android.os.Handler;
import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.view.animation.Animation;
import android.view.animation.AnimationUtils;
import android.widget.Button;
import android.widget.EditText;
import android.widget.LinearLayout;
import android.widget.TextView;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;

public class MainActivity extends AppCompatActivity {

    public static String defqultUrl = "http://m.naver.com";
    Handler handler = new Handler();

    boolean isPageOpen = false;

    Animation translateLeftAnim;
    Animation translateRightAnim;

    LinearLayout slidingpage1;
    Button button1;

    TextView txtmsg;
    Button requestbtn;
    EditText input1;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        slidingpage1 = (LinearLayout) findViewById(R.id.sliding1);

        translateLeftAnim = AnimationUtils.loadAnimation(this,R.anim.trans_left);
        translateRightAnim = AnimationUtils.loadAnimation(this,R.anim.trans_right);

        SlidingPageAnimationListener animlisten= new SlidingPageAnimationListener();
        translateLeftAnim.setAnimationListener(animlisten);
        translateRightAnim.setAnimationListener(animlisten);
        button1= (Button) findViewById(R.id.button1);
        button1.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                if (isPageOpen){
                    slidingpage1.startAnimation(translateRightAnim);
                }else{
                    slidingpage1.setVisibility(View.VISIBLE);
                    slidingpage1.startAnimation(translateLeftAnim);
                }
            }
        });
        requestbtn = (Button) findViewById(R.id.requestBtn);
        requestbtn.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                input1= (EditText) findViewById(R.id.input1);
                String urlStr= input1.getText().toString();
                ConnectThread thread= new ConnectThread(urlStr);
                thread.start();
            }
        });
    }
    class ConnectThread extends Thread{
        String urlStr;

        public ConnectThread(String instr){
            urlStr=instr;
        }

        @Override
        public void run() {
            try {
                txtmsg= (TextView) findViewById(R.id.txtMsg);
                final String output =request(urlStr);
                handler.post(new Runnable() {
                    @Override
                    public void run() {
                        txtmsg.setText(output);
                    }
                });
            }catch(Exception ex){
                ex.printStackTrace();
            }
        }
        private String request(String urlStr){
            StringBuilder output= new StringBuilder();
            try{
                URL url = new URL(urlStr);
                HttpURLConnection conn = (HttpURLConnection) url.openConnection();
                if(conn!=null){
                    conn.setConnectTimeout(10000);
                    conn.setRequestMethod("GET");
                    conn.setDoInput(true);
                    conn.setDoOutput(true);

                    int resCode = conn.getResponseCode();
                    if (resCode == HttpURLConnection.HTTP_OK){
                        BufferedReader reader = new BufferedReader(new InputStreamReader(conn.getInputStream()));
                        String line =null;
                        while(true){
                            line=reader.readLine();
                            if(line==null){
                                break;
                            }
                            output.append(line+"\n");
                        }
                        reader.close();
                        conn.disconnect();
                    }
                }
            }catch (Exception ex){
                Log.e("SampleHttp","Exception in processing response",ex);
                ex.printStackTrace();
            }
            return output.toString();
        }
    }
    private class SlidingPageAnimationListener implements Animation.AnimationListener{
        @Override
        public void onAnimationStart(Animation animation) {

        }

        @Override
        public void onAnimationRepeat(Animation animation) {

        }

        @Override
        public void onAnimationEnd(Animation animation) {
            if (isPageOpen){
                slidingpage1.setVisibility(View.INVISIBLE);

                button1.setText("Open");
                isPageOpen=false;
            }else{
                button1.setText("Closed");
                isPageOpen= true;
            }
        }
    }
}
